/// Module for testing the `setup` command.
///
/// This module contains tests to verify the correct execution of the `setup`
/// command, ensuring that the required preliminary commands (`circuit` and
/// `compile`) are executed, and that the `setup` command produces the expected
/// output files. Additionally, it provides cleanup functionality to remove
/// generated files after the test.
#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;
    use std::path::Path;

    /// Tests the `setup` command execution.
    ///
    /// This test ensures that the `setup` command executes the ceremony
    /// procedure correctly, generating the expected output files.
    /// It also verifies that the preliminary `circuit` and `compile` commands
    /// are executed as required.
    #[test]
    fn test_setup_command() {
        // Arrange: Ensure `circuit` and `compile` commands are executed
        let current_dir = std::env::current_dir().unwrap();
        execute_circuit_and_compile_commands();

        // Act: Execute the `setup` command
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("setup").assert().success();

        // Assert: Check that the expected output files are created
        assert_files_created();

        // Clean up generated files
        cleanup_files(&current_dir);
    }

    /// Executes the `circuit` and `compile` commands.
    ///
    /// This function ensures that the preliminary `circuit` and `compile`
    /// commands are executed successfully before testing the `setup` command.
    fn execute_circuit_and_compile_commands() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("circuit").assert().success();
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("compile").assert().success();
    }

    /// Checks that the expected output files are created.
    ///
    /// This function verifies that all the expected output files are created
    /// by the `setup` command.
    fn assert_files_created() {
        let output_files = vec![
            "circuit.circom",
            "circuit.r1cs",
            "circuit.sym",
            "pot12_0000.ptau",
            "pot12_0001.ptau",
            "pot12_final.ptau",
            "circuit_0000.zkey",
            "circuit_0001.zkey",
            "verification_key.json",
        ];
        for file in output_files {
            assert!(Path::new(file).exists(), "File {} does not exist", file);
        }
    }

    /// Cleans up the generated files.
    ///
    /// This function deletes all the output files generated during the test,
    /// to ensure a clean state for subsequent tests or runs.
    fn cleanup_files(dir: &Path) {
        let output_files = vec![
            "circuit.circom",
            "circuit.r1cs",
            "circuit.sym",
            "pot12_0000.ptau",
            "pot12_0001.ptau",
            "pot12_final.ptau",
            "circuit_0000.zkey",
            "circuit_0001.zkey",
            "verification_key.json",
        ];
        for file in output_files {
            let _ = fs::remove_file(file);  // Ignore any deletion errors
        }
        // Also clean up any generated directories
        fs::remove_dir_all(dir.join("circuit_js")).unwrap();
    }
}
