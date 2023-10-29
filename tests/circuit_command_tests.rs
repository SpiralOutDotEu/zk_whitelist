#[cfg(test)]
/// Module for testing the circuit command functionality.
mod tests {
    use assert_cmd::Command;
    use std::fs;
    use std::path::Path;

    /// Test to ensure the circuit command creates the circuit file with correct content
    #[test]
    fn test_circuit_command() {
        // Arrange
        let current_dir = std::env::current_dir().unwrap();
        let circuit_path = current_dir.join("circuit.circom");
        let expected_content = include_bytes!("../templates/circuit.circom");

        // Act: Execute the circuit command
        execute_circuit_command();

        // Assert: Verify the content of the created file
        let actual_content = fs::read(&circuit_path).unwrap();
        assert_files_match(expected_content, &actual_content, &circuit_path);

        // Clean up: Remove the created circuit.circom file
        fs::remove_file(circuit_path).unwrap();
    }

    // Function to execute the circuit command
    fn execute_circuit_command() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("circuit").assert().success();
    }

    // Function to compare the content of the files
    fn assert_files_match(expected_content: &[u8], actual_content: &[u8], file_path: &Path) {
        assert!(file_path.exists());
        assert_eq!(expected_content, actual_content);
    }
}
