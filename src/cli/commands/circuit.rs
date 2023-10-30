use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Copies a circuit file template to the current directory.
///
/// This function is intended to be called when the `circuit` subcommand is used.
/// It locates the current working directory, constructs a path for the new file,
/// creates a new file named `circuit.circom` in the current directory,
/// and writes the contents of a template file into the new file.
///
/// # Errors
/// Returns an `io::Result` wrapping any I/O error that occurs.
fn copy_circuit_file() -> io::Result<()> {
    // Obtain the current working directory
    let current_dir = env::current_dir()?;
    // Construct a path for the new circuit file
    let circuit_path = Path::new(&current_dir).join("circuit.circom");
    // Create a new file at the constructed path
    let mut file = File::create(circuit_path)?;
    // Write the contents of the template file into the new file
    file.write_all(include_bytes!("../../../templates/circuit.circom"))?;
    Ok(())
}

/// Handles the `circuit` CLI subcommand.
///
/// This function acts as a handler for the `circuit` subcommand.
/// It calls the `copy_circuit_file` function to perform the actual work.
///
/// # Returns
/// Returns an `io::Result` to indicate success or any I/O error that occurs.
pub fn handle_circuit_subcommand() -> std::io::Result<()> {
    copy_circuit_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    /// Tests the functionality of the `handle_circuit_subcommand` function.
    ///
    /// This test ensures that the `handle_circuit_subcommand` function correctly
    /// creates a new `circuit.circom` file in the current directory.
    ///
    /// # Returns
    /// Returns an `io::Result` to indicate the success or failure of the test.
    #[test]
    fn test_handle_circuit_subcommand() -> std::io::Result<()> {
        // Execute the function under test
        handle_circuit_subcommand()?;

        // Obtain the current working directory
        let current_dir = std::env::current_dir()?;
        // Construct the path of the circuit file
        let circuit_path = Path::new(&current_dir).join("circuit.circom");
        // Assert that the file has been created
        assert!(circuit_path.exists());

        // Clean up by removing the created file
        fs::remove_file(circuit_path)?;

        Ok(())
    }
}
