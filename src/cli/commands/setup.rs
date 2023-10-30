use fake::faker::lorem::en::Sentence;
use fake::Fake;
use std::io;
use std::process::{Command, ExitStatus};

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
pub fn execute_setup_command() -> io::Result<()> {
    // Step 1: Start Ceremony
    println!("Starting Ceremony...");
    let status = Command::new("snarkjs")
        .args(&["powersoftau", "new", "bn128", "12", "pot12_0000.ptau", "-v"])
        .status()?;
    ensure_success(status)?;

    // Step 2: Contribute to Ceremony
    println!("Contributing to Ceremony...");
    let status = Command::new("snarkjs")
        .args(&[
            "powersoftau",
            "contribute",
            "pot12_0000.ptau",
            "pot12_0001.ptau",
            "--name=\"First contribution\"",
            "-v",
            "-e=\"some random text\"",
        ])
        .status()?;
    ensure_success(status)?;

    // Step3: Prepare Phase 2
    println!("Preparing Phase 2...");
    let status = Command::new("snarkjs")
        .args(&[
            "powersoftau",
            "prepare",
            "phase2",
            "pot12_0001.ptau",
            "pot12_final.ptau",
            "-v",
        ])
        .status()?;
    ensure_success(status)?;

    // Step4: Generate zkey
    println!("Generating zkey...");
    let status = Command::new("snarkjs")
        .args(&[
            "groth16",
            "setup",
            "circuit.r1cs",
            "pot12_final.ptau",
            "circuit_0000.zkey",
        ])
        .status()?;
    ensure_success(status)?;

    // Step5: Contribute to Phase 2
    println!("Contributing to Phase 2...");
    // Generate random name and text
    let random_name: String = Sentence(2..3).fake();
    let random_text: String = Sentence(3..4).fake();

    let status = Command::new("snarkjs")
        .args(&[
            "zkey",
            "contribute",
            "circuit_0000.zkey",
            "circuit_0001.zkey",
            &format!("--name=\"{}\"", random_name),
            "-v",
            &format!("-e=\"{}\"", random_text),
        ])
        .status()?;
    ensure_success(status)?;

    // Step 6: Export the verification Key
    println!("Exporting the verification key...");
    let status = Command::new("snarkjs")
        .args(&[
            "zkey",
            "export",
            "verificationkey",
            "circuit_0001.zkey",
            "verification_key.json",
        ])
        .status()?;
    ensure_success(status)?;

    Ok(())
}

/// Ensures that a command executed successfully.
///
/// Checks the exit status of a command and returns an error if the command
/// did not execute successfully.
///
/// # Parameters
///
/// - `status`: The exit status of the command.
///
/// # Errors
///
/// Returns an error if the command did not execute successfully.
fn ensure_success(status: ExitStatus) -> io::Result<()> {
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Command execution failed",
        ));
    }
    Ok(())
}

/// Handles CLI sub command
pub fn handle_setup_subcommand() -> io::Result<()> {
    execute_setup_command()
}
