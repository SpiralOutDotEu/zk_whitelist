use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--version".to_string()) || args.contains(&"--v".to_string()) {
        println!("Zero Knowledge Whitelist Tool, version {}", env!("CARGO_PKG_VERSION"));
    }
}
