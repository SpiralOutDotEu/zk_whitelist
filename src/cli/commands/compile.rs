use crate::utils::command_runner::CommandRunner;
use std::io;

pub fn handle_compile_subcommand<R: CommandRunner>(runner: R) -> io::Result<()> {
    let args = ["circuit.circom", "--r1cs", "--sym", "--wasm"];
    runner
        .run("circom", &args)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::command_runner::MockCommandRunner;

    #[test]
    fn test_handle_compile_subcommand() {
        let runner = MockCommandRunner;
        let result = handle_compile_subcommand(runner);
        assert!(result.is_ok());
    }
}
