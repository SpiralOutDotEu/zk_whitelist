#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_compile_command() {
        // Arrange
        let current_dir = std::env::current_dir().unwrap();
        let r1cs_path = Path::new(&current_dir).join("circuit.r1cs");
        let sym_path = Path::new(&current_dir).join("circuit.sym");
        let wasm_dir_path = Path::new(&current_dir).join("circuit_js");
        let wasm_path = Path::new(&current_dir).join("circuit_js/circuit.wasm");

        // Act: Execute the circuit command to create circuit.circom
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("circuit")
            .assert()
            .success();

        // Reset the command object
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Act: Execute the compile command
        cmd.arg("compile")
            .assert()
            .success();

        // Assert
        assert!(r1cs_path.exists());
        assert!(sym_path.exists());
        assert!(wasm_path.exists());
        assert!(wasm_dir_path.exists());

        // Clean up: Remove the created files
        fs::remove_file(Path::new(&current_dir).join("circuit.circom")).expect("Failed to remove circuit.circom");
        fs::remove_file(r1cs_path).expect("Failed to remove circuit.r1cs");
        fs::remove_file(sym_path).expect("Failed to remove circuit.sym");
        fs::remove_file(wasm_path).expect("Failed to remove circuit.wasm");
        fs::remove_dir_all(wasm_dir_path).expect("Failed to remove circuit_js directory");
    }
}
