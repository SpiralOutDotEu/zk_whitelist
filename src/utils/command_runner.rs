use std::io;
use std::sync::{Arc, Mutex};

/// A trait defining a generic command runner interface.
///
/// Implementations of this trait can be used to execute
/// shell commands and collect the results.
pub trait CommandRunner {
    /// Executes a shell command with the specified arguments.
    ///
    /// # Parameters
    /// - `command`: The name of the command to execute.
    /// - `args`: A slice of arguments to pass to the command.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the command execution.
    fn run(&self, command: &str, args: &[&str]) -> Result<(), String>;
}

/// A real implementation of the `CommandRunner` trait that executes shell commands.
pub struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, command: &str, args: &[&str]) -> Result<(), String> {
        // Execute the command with the specified arguments
        let output = std::process::Command::new(command)
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;

        // Check the exit status of the command
        if !output.status.success() {
            return Err(format!("Command failed: {:?}", output));
        }

        Ok(())
    }
}

/// A mock implementation of the `CommandRunner` trait for testing purposes.
///
/// This implementation records the commands and arguments it was called with,
/// allowing tests to verify correct behavior without executing real shell commands.
pub struct MockCommandRunner {
    calls: Arc<Mutex<Vec<(String, Vec<String>)>>>,
}

impl MockCommandRunner {
    /// Creates a new `MockCommandRunner` instance.
    pub fn new() -> Self {
        MockCommandRunner {
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns a list of command and argument tuples that this runner was called with.
    pub fn calls(&self) -> Vec<(String, Vec<String>)> {
        self.calls.lock().unwrap().clone()
    }
}

impl CommandRunner for MockCommandRunner {
    fn run(&self, command: &str, args: &[&str]) -> Result<(), String> {
        // Record the command and arguments
        self.calls.lock().unwrap().push((
            command.to_string(),
            args.iter().map(|&s| s.to_string()).collect(),
        ));
        Ok(())
    }
}

/// Executes a `snarkjs` command with the specified arguments using the provided `CommandRunner`.
///
/// # Parameters
/// - `runner`: The `CommandRunner` implementation to use for executing the command.
/// - `args`: A slice of arguments to pass to the `snarkjs` command.
///
/// # Returns
/// A `Result` indicating the success or failure of the command execution.
pub fn run_snarkjs_command<R: CommandRunner>(runner: &R, args: &[&str]) -> io::Result<()> {
    runner
        .run("snarkjs", args)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test to verify the execution of `snarkjs` command using `MockCommandRunner`.
    fn test_run_snarkjs_command() {
        let mock_runner = MockCommandRunner::new();
        let args = &["arg1", "arg2"];

        let result = run_snarkjs_command(&mock_runner, args);

        assert!(result.is_ok());
        assert_eq!(
            mock_runner.calls(),
            vec![(
                "snarkjs".to_string(),
                vec!["arg1".to_string(), "arg2".to_string()]
            )]
        );
    }
}
