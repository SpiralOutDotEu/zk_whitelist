use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--version".to_string()) || args.contains(&"--v".to_string()) {
        println!(
            "Zero Knowledge Whitelist Tool, version {}",
            env!("CARGO_PKG_VERSION")
        );
    } else if args.contains(&"circuit".to_string()) {
        let current_dir = env::current_dir()?;
        let circuit_path = Path::new(&current_dir).join("circuit.circom");
        let mut file = File::create(circuit_path)?;
        file.write_all(include_bytes!("../templates/circuit.circom"))?;
    }

    Ok(())
}
