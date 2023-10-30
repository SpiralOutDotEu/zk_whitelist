use crate::utils::command_runner::{run_snarkjs_command, CommandRunner};
use std::io;

/// Executes the setup procedure to generate necessary files for Zero-Knowledge (ZK) proofs.
///
/// This function orchestrates the ceremony procedure to create the necessary
/// files for Zero-Knowledge proofs. It performs several steps, each invoking
/// an external command via the `snarkjs` utility. Basic progress reporting is
/// provided via print statements.
///
/// # Parameters
/// - `runner`: A reference to an implementation of the `CommandRunner` trait which will execute the `snarkjs` commands.
/// - `random_name`: A `String` containing a random name used in the ceremony contributions.
/// - `random_text`: A `String` containing a random text used in the ceremony contributions.
///
/// # Errors
/// Returns an error if any of the external commands fail.
pub fn execute_setup_command<R: CommandRunner>(
    runner: &R,
    random_name: String,
    random_text: String,
) -> io::Result<()> {
    // Various steps in the setup procedure are detailed below:

    // Step 1: Start Ceremony
    println!("Starting Ceremony...");
    start_ceremony(runner)?;

    // Step 2: Contribute to Ceremony
    println!("Contributing to Ceremony...");
    contribute_to_ceremony(runner, random_name.clone(), random_text.clone())?;

    // Step3: Prepare Phase 2
    println!("Preparing Phase 2 (this takes some time)...");
    prepare_phase_2(runner)?;

    // Step4: Generate zkey
    println!("Generating zkey...");
    generate_zkey(runner)?;

    // Step5: Contribute to Phase 2
    println!("Contributing to Phase 2...");
    contribute_to_phase_2(runner, random_name, random_text)?;

    // Step 6: Export the verification Key
    println!("Exporting the verification key...");
    export_verification_key(runner)?;
    println!("Ceremony completed");
    Ok(())
}

// The following helper functions represent individual steps in the setup procedure:

// Step 1: Start Ceremony
fn start_ceremony<R: CommandRunner>(runner: &R) -> Result<(), io::Error> {
    run_snarkjs_command(
        runner,
        &["powersoftau", "new", "bn128", "12", "pot12_0000.ptau", "-v"],
    )?;
    Ok(())
}

// Step 2: Contribute to Ceremony
fn contribute_to_ceremony<R: CommandRunner>(
    runner: &R,
    random_name: String,
    random_text: String,
) -> Result<(), io::Error> {
    run_snarkjs_command(
        runner,
        &[
            "powersoftau",
            "contribute",
            "pot12_0000.ptau",
            "pot12_0001.ptau",
            &format!("--name=\"{}\"", random_name),
            "-v",
            &format!("-e=\"{}\"", random_text),
        ],
    )?;
    Ok(())
}

// Step3: Prepare Phase 2
fn prepare_phase_2<R: CommandRunner>(runner: &R) -> Result<(), io::Error> {
    run_snarkjs_command(
        runner,
        &[
            "powersoftau",
            "prepare",
            "phase2",
            "pot12_0001.ptau",
            "pot12_final.ptau",
            "-v",
        ],
    )?;
    Ok(())
}

// Step4: Generate zkey
fn generate_zkey<R: CommandRunner>(runner: &R) -> Result<(), io::Error> {
    run_snarkjs_command(
        runner,
        &[
            "groth16",
            "setup",
            "circuit.r1cs",
            "pot12_final.ptau",
            "circuit_0000.zkey",
        ],
    )?;
    Ok(())
}

// Step5: Contribute to Phase 2
fn contribute_to_phase_2<R: CommandRunner>(
    runner: &R,
    random_name: String,
    random_text: String,
) -> Result<(), io::Error> {
    run_snarkjs_command(
        runner,
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
    Ok(())
}

// Step 6: Export the verification Key
fn export_verification_key<R: CommandRunner>(runner: &R) -> Result<(), io::Error> {
    run_snarkjs_command(
        runner,
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
///
/// This function serves as an entry point to handle the setup subcommand. It calls the
/// `execute_setup_command` function passing the necessary arguments.
///
/// # Parameters
/// - `runner`: A reference to an implementation of the `CommandRunner` trait which will execute the `snarkjs` commands.
/// - `random_name`: A `String` containing a random name used in the ceremony contributions.
/// - `random_text`: A `String` containing a random text used in the ceremony contributions.
///
/// # Returns
/// Returns an `io::Result` to indicate success or any I/O error that occurs.
pub fn handle_setup_subcommand<R: CommandRunner>(
    runner: &R,
    random_name: String,
    random_text: String,
) -> io::Result<()> {
    execute_setup_command(runner, random_name, random_text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use fake::{faker::lorem::en::Sentence, Fake};

    use super::*;
    use crate::utils::command_runner::MockCommandRunner;

    /// Tests the `execute_setup_command` function to ensure it orchestrates the setup procedure correctly.
    ///
    /// This test creates a `MockCommandRunner` instance to simulate the behavior of the `CommandRunner` trait.
    /// It then calls `handle_setup_subcommand` with this mock runner and random name and text,
    /// checking the returned result to ensure it is `Ok`. Finally, it checks the command and arguments passed to
    /// the `run` method of the `MockCommandRunner` to ensure they match the expected commands and arguments for
    /// the `snarkjs` utility.
    #[test]
    fn test_execute_setup_command() {
        let runner = MockCommandRunner::new();
        let random_name: String = Sentence(2..3).fake();
        let random_text: String = Sentence(3..4).fake();
        let result = handle_setup_subcommand(&runner, random_name.clone(), random_text.clone());
        assert!(result.is_ok());

        let expected_calls = vec![
            (
                "snarkjs".to_string(),
                vec![
                    "powersoftau".to_string(),
                    "new".to_string(),
                    "bn128".to_string(),
                    "12".to_string(),
                    "pot12_0000.ptau".to_string(),
                    "-v".to_string(),
                ],
            ),
            (
                "snarkjs".to_string(),
                vec![
                    "powersoftau".to_string(),
                    "contribute".to_string(),
                    "pot12_0000.ptau".to_string(),
                    "pot12_0001.ptau".to_string(),
                    format!("--name=\"{}\"", random_name),
                    "-v".to_string(),
                    format!("-e=\"{}\"", random_text),
                ],
            ),
            (
                "snarkjs".to_string(),
                vec![
                    "powersoftau".to_string(),
                    "prepare".to_string(),
                    "phase2".to_string(),
                    "pot12_0001.ptau".to_string(),
                    "pot12_final.ptau".to_string(),
                    "-v".to_string(),
                ],
            ),
            (
                "snarkjs".to_string(),
                vec![
                    "groth16".to_string(),
                    "setup".to_string(),
                    "circuit.r1cs".to_string(),
                    "pot12_final.ptau".to_string(),
                    "circuit_0000.zkey".to_string(),
                ],
            ),
            (
                "snarkjs".to_string(),
                vec![
                    "zkey".to_string(),
                    "contribute".to_string(),
                    "circuit_0000.zkey".to_string(),
                    "circuit_0001.zkey".to_string(),
                    format!("--name=\"{}\"", random_name),
                    "-v".to_string(),
                    format!("-e=\"{}\"", random_text),
                ],
            ),
            (
                "snarkjs".to_string(),
                vec![
                    "zkey".to_string(),
                    "export".to_string(),
                    "verificationkey".to_string(),
                    "circuit_0001.zkey".to_string(),
                    "verification_key.json".to_string(),
                ],
            ),
        ];

        assert_eq!(runner.calls(), expected_calls);
    }
}
