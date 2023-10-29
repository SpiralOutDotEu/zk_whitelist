#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_circuit_command() {
        // Arrange
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let circuit_path = Path::new(&current_dir).join("circuit.circom");
        let expected_content = include_bytes!("../templates/circuit.circom");

        // Act
        cmd.arg("circuit")
            .assert()
            .success();
        let actual_content = fs::read(circuit_path.clone()).unwrap();

        // Assert
        assert!(circuit_path.exists());
        assert_eq!(expected_content, actual_content.as_slice());

        // Clean up: Remove the created circuit.circom file
        fs::remove_file(circuit_path).expect("Failed to remove circuit.circom");
    }
}
