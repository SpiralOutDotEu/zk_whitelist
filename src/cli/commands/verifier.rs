use crate::utils::command_runner::{run_snarkjs_command, CommandRunner};
use std::io;

/// Handles the `verifier` subcommand by executing the `snarkjs generateverifier` shell command.
///
/// This function accepts a generic parameter `R` which implements the `CommandRunner` trait,
/// allowing for the execution of the shell command to be handled by `runner`.
///
/// # Parameters
/// - `runner`: The command runner which will execute the shell command.
///
/// # Returns
/// - An `io::Result<()>` which will be `Ok(())` if the command executes successfully, or an `Err`
///   wrapping an `io::Error` if an error occurs.
pub fn handle_verifier_subcommand<R: CommandRunner>(runner: &R) -> io::Result<()> {
    // Call the run_snarkjs_command function with the runner and the arguments for the shell command.
    run_snarkjs_command(
        runner,
        &[
            "zkey",
            "export",
            "solidityverifier",
            "circuit_0001.zkey",
            "verifier.sol",
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::command_runner::MockCommandRunner;

    /// Tests the `handle_verifier_subcommand` function to ensure it can execute the
    /// `snarkjs generateverifier` shell command.
    ///
    /// This test creates a `MockCommandRunner` instance to simulate the behavior of the `CommandRunner` trait.
    /// It then calls `handle_verifier_subcommand` with this mock runner, checks the returned result to ensure
    /// it is `Ok`, and finally, it checks the command and arguments passed to the `run` method of the `MockCommandRunner`
    /// to ensure they match the expected command and arguments for the `snarkjs` utility.
    #[test]
    fn test_handle_verifier_subcommand() {
        // Create a new MockCommandRunner instance for testing.
        let runner = MockCommandRunner::new();

        // Call the function with the mock runner.
        let result = handle_verifier_subcommand(&runner); // Now passing a reference

        // Assert the result is Ok(()) indicating success.
        assert!(result.is_ok());

        // Assert that the `run` method of `MockCommandRunner` has been called with correct arguments
        assert_eq!(
            runner.calls(),
            vec![(
                "snarkjs".to_string(),
                vec![
                    "zkey".to_string(),
                    "export".to_string(),
                    "solidityverifier".to_string(),
                    "circuit_0001.zkey".to_string(),
                    "verifier.sol".to_string(),
                ]
            )]
        );
    }
}
