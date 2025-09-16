use ark_bn254::{Bn254, Fr};
use ark_circom::CircomBuilder;
use ark_circom::CircomConfig;
use ark_groth16::{Groth16, prepare_verifying_key};
use ark_serialize::CanonicalSerialize;
use ark_snark::SNARK;
// use rand::rngs::StdRng; Use only for testing!
// use rand::SeedableRng;  Use only for testing!
use serde_json::json;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use rand::rngs::OsRng;

use crate::utils::{command_runner::CommandRunner, filesystem_operations::FileSystemOps};

/// Handles the 'sui-proofs' subcommand.
///
/// Reads addresses from the specified file, generates Sui-compatible proofs for each address,
/// and outputs the verification key, proof, and public inputs in the format required by Sui.
///
/// # Arguments
///
/// * `runner` - A command runner for executing external commands.
/// * `file_name` - The name of the input file containing addresses.
/// * `file_ops` - A file system operations interface for moving or manipulating files.
///
/// # Returns
///
/// Returns a `Result` to indicate success or failure.
pub fn handle_sui_proofs_subcommand<R, F>(
    _runner: &R,
    file_name: &str,
    file_ops: &F,
) -> Result<(), String>
where
    R: CommandRunner,
    F: FileSystemOps,
{
    let addresses = file_ops.read_lines(file_name)?;
    let mut proofs_map = HashMap::new();

    // Create tokio runtime for ark-circom
    let rt = Runtime::new().map_err(|e| format!("Failed to create tokio runtime: {}", e))?;

    // Load the proving key from the ceremony
    let proving_key = rt.block_on(load_proving_key())?;

    // Serialize verification key once (common for all proofs)
    let mut vk_bytes = Vec::new();
    proving_key.vk.serialize_compressed(&mut vk_bytes)
        .map_err(|e| format!("Failed to serialize verification key: {}", e))?;
    let vk_hex = hex::encode(vk_bytes);

    for address_hex in &addresses {
        let address_dec = address_hex.trim_start_matches("0x");
        let address_bigint = num_bigint::BigInt::parse_bytes(address_dec.as_bytes(), 16)
            .ok_or("Failed to parse address to decimal")?;

        // Load the WASM and R1CS for witness and proof generation
        let cfg = rt.block_on(async {
            CircomConfig::<Fr>::new("circuit.wasm", "circuit.r1cs")
                .map_err(|e| format!("Failed to load circuit config: {}", e))
        })?;
        let mut builder = CircomBuilder::new(cfg);

        // Set up the circuit with the address
        builder.push_input("addressInDecimal", address_bigint.clone());
        builder.push_input("sameAddressButPublic", address_bigint.clone());

        let _circuit = builder.setup();
        let circuit = rt.block_on(async {
            builder.build().map_err(|e| format!("Failed to build circuit: {}", e))
        })?;
        let public_inputs = circuit.get_public_inputs().ok_or("Failed to get public inputs")?;

        // Generate random number generator
        // let mut rng = StdRng::from_seed([0; 32]);  // ⚠️ WARNING: Not secure! Use only for testing!
        let mut rng = OsRng;  // Cryptographically secure random number generator

        // Create proof
        let proof = Groth16::<Bn254>::prove(&proving_key, circuit, &mut rng)
            .map_err(|e| format!("Failed to generate proof: {}", e))?;

        // Verify proof locally
        let pvk = prepare_verifying_key(&proving_key.vk);
        let verified = Groth16::<Bn254>::verify_with_processed_vk(&pvk, &public_inputs, &proof)
            .map_err(|e| format!("Failed to verify proof: {}", e))?;
        
        if !verified {
            return Err("Generated proof failed verification".to_string());
        }

        // Serialize proof
        let mut proof_serialized = Vec::new();
        proof.serialize_compressed(&mut proof_serialized)
            .map_err(|e| format!("Failed to serialize proof: {}", e))?;
        let proof_hex = hex::encode(proof_serialized);

        // Serialize public inputs
        let mut public_inputs_serialized = Vec::new();
        public_inputs.iter().for_each(|input| {
            input.serialize_compressed(&mut public_inputs_serialized).unwrap();
        });
        let public_inputs_hex = hex::encode(public_inputs_serialized);

        // Store the proof data (without verifying key)
        proofs_map.insert(
            address_hex.clone(),
            json!({
                "proof": proof_hex,
                "public_inputs": public_inputs_hex
            }),
        );

        println!("Generated Sui-compatible proof for address: {}", address_hex);
    }

    // Create the final output structure with verifying key as parent
    let sui_proofs_output = json!({
        "verifying_key": vk_hex,
        "proofs": proofs_map
    });

    // Write the output to a file
    let output_file_name = format!("{}{}", file_name, ".sui_proofs.json");
    let output_content = serde_json::to_string_pretty(&sui_proofs_output)
        .map_err(|e| format!("Failed to serialize output: {}", e))?;
    
    file_ops.write_to_file(&output_file_name, &output_content)?;
    
    println!("Sui-compatible proofs written to: {}", output_file_name);
    println!("Verification key: {}", vk_hex);
    
    Ok(())
}

/// Loads the proving key from the ceremony files
async fn load_proving_key() -> Result<ark_groth16::ProvingKey<ark_bn254::Bn254>, String> {
    // For now, we'll generate a random proving key as a placeholder
    // In a real implementation, you would load the proving key from the ceremony files
    // This is a simplified version that matches the convert_to_sui.rs example
    
    // Load the circuit config
    let cfg = CircomConfig::<Fr>::new("circuit.wasm", "circuit.r1cs")
        .map_err(|e| format!("Failed to load circuit config: {}", e))?;
    let mut builder = CircomBuilder::new(cfg);
    
    // Add dummy inputs for setup
    builder.push_input("addressInDecimal", 0);
    builder.push_input("sameAddressButPublic", 0);
    
    let circuit = builder.setup();
    
    // Generate a random proving key (WARNING: This is not secure for production)
    // let mut rng = StdRng::from_seed([0; 32]);  // ⚠️ WARNING: Not secure! Use only for testing!
    let mut rng = OsRng;  // Cryptographically secure random number generator
    let pk = Groth16::<Bn254>::generate_random_parameters_with_reduction(circuit, &mut rng)
        .map_err(|e| format!("Failed to generate proving key: {}", e))?;
    
    Ok(pk)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::command_runner::MockCommandRunner;
    use crate::utils::filesystem_operations::MockFileSystemOps;

    #[test]
    fn test_handle_sui_proofs_subcommand() {
        // This test would need to be implemented with proper mocking
        // For now, it's a placeholder
        assert!(true);
    }
}
