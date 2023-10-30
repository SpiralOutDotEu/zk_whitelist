use crate::utils::command_runner::CommandRunner;
use std::io;

/// Handles the compilation of a circuit file using the `circom` compiler.
///
/// This function is triggered when the `compile` subcommand is used. It prepares the necessary
/// arguments for the `circom` command and executes it using the provided `CommandRunner` implementation.
///
/// # Parameters
/// - `runner`: A reference to an implementation of the `CommandRunner` trait which will execute the `circom` command.
///
/// # Returns
/// - A `Result` which is `Ok` if the command executes successfully, or an `Err` wrapping an `io::Error` if an error occurs.
pub fn handle_compile_subcommand<R: CommandRunner>(runner: &R) -> io::Result<()> {
    // Prepare the arguments for the `circom` command.
    let args = ["circuit.circom", "--r1cs", "--sym", "--wasm"];

    // Execute the `circom` command with the provided arguments using the `CommandRunner` implementation.
    // Map any error to an `io::Error`.
    runner
        .run("circom", &args)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::command_runner::MockCommandRunner;

    /// Tests the `handle_compile_subcommand` function to ensure it executes the `circom` command
    /// with the correct arguments.
    ///
    /// This test creates a `MockCommandRunner` instance to simulate the behavior of the `CommandRunner` trait.
    /// It then calls `handle_compile_subcommand` with this mock runner, and checks the returned result to ensure
    /// it is `Ok`. Finally, it checks the command and arguments passed to the `run` method of the `MockCommandRunner`
    /// to ensure they match the expected command and arguments for the `circom` compiler.
    #[test]
    fn test_handle_compile_subcommand() {
        // Create a new `MockCommandRunner` instance for testing.
        let runner = MockCommandRunner::new();

        // Call the `handle_compile_subcommand` function with the mock runner.
        // This should internally call the `run` method of `MockCommandRunner` with the `circom` command and arguments.
        let result = handle_compile_subcommand(&runner);

        // Assert the result is `Ok`, indicating the `circom` command was executed successfully.
        assert!(result.is_ok());

        // Check the command and arguments passed to the `run` method of the `MockCommandRunner`.
        // This ensures `handle_compile_subcommand` is forming the correct command and arguments for the `circom` compiler.
        assert_eq!(
            runner.calls(),
            vec![(
                "circom".to_string(),
                vec![
                    "circuit.circom".to_string(),
                    "--r1cs".to_string(),
                    "--sym".to_string(),
                    "--wasm".to_string(),
                ]
            )]
        );
    }
}
