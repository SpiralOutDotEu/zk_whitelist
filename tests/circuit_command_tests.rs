#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_circuit_command() {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let current_dir = temp_dir.path().to_path_buf();
        let expected_content = include_bytes!("../templates/circuit.circom");

        // Act
        let circuit_path = execute_circuit_command(&current_dir);
        let actual_content = fs::read(&circuit_path).unwrap();

        // Assert
        assert_files_match(expected_content, &actual_content, &circuit_path);

    }

    fn execute_circuit_command(dir: &PathBuf) -> PathBuf {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.current_dir(dir)
            .arg("circuit")
            .assert()
            .success();
        dir.join("circuit.circom")
    }

    fn assert_files_match(expected_content: &[u8], actual_content: &[u8], file_path: &PathBuf) {
        assert!(file_path.exists());
        assert_eq!(expected_content, actual_content);
    }
}
