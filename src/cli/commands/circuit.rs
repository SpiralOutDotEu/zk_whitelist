use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Copies a circuit file template to the current directory.
/// This function is called when the `circuit` subcommand is used.
fn copy_circuit_file() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let circuit_path = Path::new(&current_dir).join("circuit.circom");
    let mut file = File::create(circuit_path)?;
    file.write_all(include_bytes!("../../../templates/circuit.circom"))?;
    Ok(())
}

/// Handles CLI sub command
pub fn handle_circuit_subcommand() -> std::io::Result<()> {
    copy_circuit_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_handle_circuit_subcommand() -> std::io::Result<()> {
        handle_circuit_subcommand()?;

        let current_dir = std::env::current_dir()?;
        let circuit_path = Path::new(&current_dir).join("circuit.circom");
        assert!(circuit_path.exists());

        // Clean up
        fs::remove_file(circuit_path)?;

        Ok(())
    }
}
