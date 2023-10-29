use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Entry point of the application.
///
/// This function processes command line arguments to determine which action to perform.
fn main() -> io::Result<()> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // Check for version flag and display version if present
    if args.contains(&"--version".to_string()) || args.contains(&"--v".to_string()) {
        display_version();
    }
    // Check for circuit command and copy circuit file if present
    else if args.contains(&"circuit".to_string()) {
        copy_circuit_file()?;
    }
    // Check for compile command and compile circuit if present
    else if args.contains(&"compile".to_string()) {
        compile_circuit()?;
    }

    Ok(())
}

/// Displays the version of the tool to the console.
fn display_version() {
    println!(
        "Zero Knowledge Whitelist Tool, version {}",
        env!("CARGO_PKG_VERSION")
    );
}

/// Copies the circuit file template to the current directory.
///
/// # Errors
///
/// Returns an error if there is a problem accessing the current directory or writing the file.
fn copy_circuit_file() -> io::Result<()> {
    // Getting the current directory
    let current_dir = env::current_dir()?;
    // Defining the path for the circuit file
    let circuit_path = Path::new(&current_dir).join("circuit.circom");
    // Creating and writing to the circuit file
    let mut file = File::create(circuit_path)?;
    file.write_all(include_bytes!("../templates/circuit.circom"))?;
    Ok(())
}

/// Compiles the circuit file using the circom compiler.
///
/// # Errors
///
/// Returns an error if the compilation fails.
fn compile_circuit() -> io::Result<()> {
    // Getting the current directory
    let current_dir = env::current_dir()?;
    // Executing the circom compiler with required arguments
    let output = Command::new("circom")
        .current_dir(&current_dir)
        .arg("circuit.circom")
        .args(&["--r1cs", "--sym", "--wasm"])
        .output()?;

    // Checking for compilation success
    if !output.status.success() {
        let error_message = format!(
            "Compilation failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(io::Error::new(io::ErrorKind::Other, error_message));
    }

    Ok(())
}
