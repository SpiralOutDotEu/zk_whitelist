use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub trait FileSystem {
    fn create(&self, path: &Path) -> std::io::Result<Box<dyn std::io::Write>>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create(&self, path: &Path) -> std::io::Result<Box<dyn std::io::Write>> {
        File::create(path).map(|file| Box::new(file) as Box<dyn std::io::Write>)
    }
}

/// Copies a circuit file template to the current directory.
/// This function is called when the `circuit` subcommand is used.
fn copy_circuit_file(fs: &dyn FileSystem) -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let circuit_path = Path::new(&current_dir).join("circuit.circom");
    let mut file = File::create(circuit_path)?;
    file.write_all(include_bytes!("../../../templates/circuit.circom"))?;
    Ok(())
}

/// Handles CLI sub command
pub fn handle_circuit_subcommand(fs: &dyn FileSystem) -> std::io::Result<()> {
    copy_circuit_file(fs)
}
