#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    #[test]
    fn test_compile_command() {
        // Arrange
        let temp_dir = tempdir().unwrap();
        let current_dir = temp_dir.path().to_path_buf();
        create_circuit_file(&current_dir).unwrap();

        // Act
        execute_compile_command(&current_dir);

        // Assert
        assert_compiled_files_exist(&current_dir);

    }

    fn create_circuit_file(dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let circuit_path = dir.join("circuit.circom");
        fs::write(&circuit_path, include_bytes!("../templates/circuit.circom"))?;
        Ok(circuit_path)
    }

    fn execute_compile_command(dir: &Path) {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.current_dir(dir)
            .arg("compile")
            .assert()
            .success();
    }

    fn assert_compiled_files_exist(dir: &Path) {
        let r1cs_path = dir.join("circuit.r1cs");
        let sym_path = dir.join("circuit.sym");
        let wasm_dir_path = dir.join("circuit_js");
        assert!(r1cs_path.exists());
        assert!(sym_path.exists());
        assert!(wasm_dir_path.exists());
    }
}
