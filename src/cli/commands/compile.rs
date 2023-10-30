use std::env;
use std::io;
use std::process::Command;

/// Compiles the circuit file using the circom compiler.
/// This function is called when the `compile` subcommand is used.
pub fn compile_circuit() -> io::Result<()> {
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

/// Handles CLI sub command
pub fn handle_compile_subcommand() -> io::Result<()> {
    compile_circuit()
}
