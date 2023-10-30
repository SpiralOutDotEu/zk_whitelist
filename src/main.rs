use clap::{Parser, Subcommand};
use fake::faker::lorem::en::Sentence;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
extern crate fake;

use fake::Fake;

/// Represents the command line interface for the Zero Knowledge Whitelist Tool.
/// Deriving `Parser` from clap allows for automatic parsing of command line arguments.
#[derive(Parser)]
#[clap(
    name = "Zero Knowledge Whitelist Tool",
    version = env!("CARGO_PKG_VERSION"),
    author = "Nikos Koumbakis <n.koumbakis@gmail.com>",
    about = "This tool orchestrates the management of an address whitelist using Zero-Knowledge (ZK) proofs.\nSimply input the addresses, and it will generate the corresponding Solidity code.\nIt streamlines the process of maintaining a secure and efficient whitelist for your decentralized application."
)]
struct Cli {
    /// The subcommand to be executed, parsed from the command line arguments.
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Enumerates the available subcommands.
/// Deriving `Subcommand` from clap provides automatic subcommand handling.
#[derive(Subcommand)]
enum SubCommand {
    /// The `circuit` subcommand copies a circuit file template to the current directory.
    Circuit,
    /// The `compile` subcommand compiles the circuit file.
    Compile,
    /// Executes the setup ceremony to generate secure setup
    Setup,
}
/// The entry point of the application.
/// Parses command line arguments and executes the corresponding subcommand.
fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.subcmd {
        SubCommand::Circuit => copy_circuit_file()?,
        SubCommand::Compile => compile_circuit()?,
        SubCommand::Setup => execute_setup_command()?,
    }

    Ok(())
}

/// Copies a circuit file template to the current directory.
/// This function is called when the `circuit` subcommand is used.
fn copy_circuit_file() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let circuit_path = Path::new(&current_dir).join("circuit.circom");
    let mut file = File::create(circuit_path)?;
    file.write_all(include_bytes!("../templates/circuit.circom"))?;
    Ok(())
}

/// Compiles the circuit file using the circom compiler.
/// This function is called when the `compile` subcommand is used.
fn compile_circuit() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    eprintln!("Working directory: {:?}", current_dir);
    let output = Command::new("circom")
        .current_dir(&current_dir)
        .arg("circuit.circom")
        .args(&["--r1cs", "--sym", "--wasm"])
        .output()?;

    eprintln!("circom stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("circom stderr: {}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Compilation failed"));
    }

    Ok(())
}

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
fn execute_setup_command() -> io::Result<()> {
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
fn ensure_success(status: std::process::ExitStatus) -> io::Result<()> {
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Command execution failed",
        ));
    }
    Ok(())
}
