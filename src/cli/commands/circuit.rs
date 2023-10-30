use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Copies a circuit file template to the current directory.
/// This function is called when the `circuit` subcommand is used.
pub fn copy_circuit_file() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let circuit_path = Path::new(&current_dir).join("circuit.circom");
    let mut file = File::create(circuit_path)?;
    file.write_all(include_bytes!("../../../templates/circuit.circom"))?;
    Ok(())
}

/// Handles CLI sub command
pub fn handle_circuit_subcommand() -> io::Result<()> {
    copy_circuit_file()
}
