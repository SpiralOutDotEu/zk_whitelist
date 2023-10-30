use crate::utils::command_runner::{run_snarkjs_command, CommandRunner};
use std::io;

/// Handles the `verifier` subcommand
pub fn handle_verifier_subcommand<R: CommandRunner>(runner: R) -> io::Result<()> {
    run_snarkjs_command(&runner, &["generateverifier"])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::command_runner::MockCommandRunner;

    #[test]
    fn test_handle_verifier_subcommand() {
        let runner = MockCommandRunner;
        let result = handle_verifier_subcommand(runner);
        assert!(result.is_ok());
    }
}
