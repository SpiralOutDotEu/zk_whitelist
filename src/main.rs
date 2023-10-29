use clap::{Parser, Subcommand};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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
}
/// The entry point of the application.
/// Parses command line arguments and executes the corresponding subcommand.
fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.subcmd {
        SubCommand::Circuit => copy_circuit_file()?,
        SubCommand::Compile => compile_circuit()?,
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
