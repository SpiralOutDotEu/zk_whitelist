use crate::utils::command_runner::{run_snarkjs_command, CommandRunner};
use fake::faker::lorem::en::Sentence;
use fake::Fake;
use std::io;

/// Executes the setup procedure to generate necessary files for ZK proofs.
///
/// This function orchestrates the ceremony procedure to create the necessary
/// files for Zero-Knowledge proofs. It performs several steps, each invoking
/// an external command via the `snarkjs` utility. Basic progress reporting is
/// provided via print statements.
///
/// # Errors
///
/// Returns an error if any of the external commands fail.
pub fn execute_setup_command<R: CommandRunner>(runner: R) -> io::Result<()> {
    // Step 1: Start Ceremony
    println!("Starting Ceremony...");
    run_snarkjs_command(
        &runner,
        &["powersoftau", "new", "bn128", "12", "pot12_0000.ptau", "-v"],
    )?;

    // Step 2: Contribute to Ceremony
    println!("Contributing to Ceremony...");
    run_snarkjs_command(
        &runner,
        &[
            "powersoftau",
            "contribute",
            "pot12_0000.ptau",
            "pot12_0001.ptau",
            "--name=\"First contribution\"",
            "-v",
            "-e=\"some random text\"",
        ],
    )?;

    // Step3: Prepare Phase 2
    println!("Preparing Phase 2...");
    run_snarkjs_command(
        &runner,
        &[
            "powersoftau",
            "prepare",
            "phase2",
            "pot12_0001.ptau",
            "pot12_final.ptau",
            "-v",
        ],
    )?;

    // Step4: Generate zkey
    println!("Generating zkey...");
    run_snarkjs_command(
        &runner,
        &[
            "groth16",
            "setup",
            "circuit.r1cs",
            "pot12_final.ptau",
            "circuit_0000.zkey",
        ],
    )?;

    // Step5: Contribute to Phase 2
    println!("Contributing to Phase 2...");
    // Generate random name and text
    let random_name: String = Sentence(2..3).fake();
    let random_text: String = Sentence(3..4).fake();

    run_snarkjs_command(
        &runner,
        &[
            "zkey",
            "contribute",
            "circuit_0000.zkey",
            "circuit_0001.zkey",
            &format!("--name=\"{}\"", random_name),
            "-v",
            &format!("-e=\"{}\"", random_text),
        ],
    )?;

    // Step 6: Export the verification Key
    println!("Exporting the verification key...");
    run_snarkjs_command(
        &runner,
        &[
            "zkey",
            "export",
            "verificationkey",
            "circuit_0001.zkey",
            "verification_key.json",
        ],
    )?;

    Ok(())
}

/// Handles CLI sub command
pub fn handle_setup_subcommand<R: CommandRunner>(runner: R) -> io::Result<()> {
    execute_setup_command(runner)
}
