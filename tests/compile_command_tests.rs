#[cfg(test)]
/// Module for testing the compile command functionality.
mod tests {
    use assert_cmd::Command;
    use std::fs;
    use std::path::Path;

    /// Test to ensure the compile command compiles the circuit file and produces the expected files
    #[test]
    fn test_compile_command() {
        // Arrange: Create the circuit file
        let current_dir = std::env::current_dir().unwrap();
        create_circuit_file(&current_dir);

        // Act: Execute the compile command
        execute_compile_command(&current_dir);

        // Assert: Verify the compiled files exist
        assert_compiled_files_exist(&current_dir);

        // Clean up: Remove the created and compiled files
        clean_up_files(&current_dir);
    }

    // Function to create a circuit file in the specified directory
    fn create_circuit_file(dir: &Path) {
        let circuit_path = dir.join("circuit.circom");
        fs::write(&circuit_path, include_bytes!("../templates/circuit.circom")).unwrap();
    }

    // Function to execute the compile command
    fn execute_compile_command(dir: &Path) {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.current_dir(dir).arg("compile").assert().success();
    }
    // Function to assert that the compiled files exist
    fn assert_compiled_files_exist(dir: &Path) {
        let r1cs_path = dir.join("circuit.r1cs");
        let sym_path = dir.join("circuit.sym");
        let wasm_dir_path = dir.join("circuit_js");
        assert!(r1cs_path.exists());
        assert!(sym_path.exists());
        assert!(wasm_dir_path.exists());
    }

    // Function to clean up the created and compiled files
    fn clean_up_files(dir: &Path) {
        fs::remove_file(dir.join("circuit.circom")).unwrap();
        fs::remove_file(dir.join("circuit.r1cs")).unwrap();
        fs::remove_file(dir.join("circuit.sym")).unwrap();
        fs::remove_dir_all(dir.join("circuit_js")).unwrap();
    }
}
