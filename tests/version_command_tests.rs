// Conditional compilation directive for running code only in test context
#[cfg(test)]
/// Module for testing the version command functionality.
mod tests {
    // Importing the necessary library for command assertion
    use assert_cmd::Command;

    /// Test function to check the version command output
    #[test]
    fn test_version_command() {
        // Creating a command object for the binary
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Executing the version command with --version flag
        // and asserting the command runs successfully and outputs the correct version
        cmd.arg("--version")
            .assert()
            .success()
            .stdout(predicates::str::contains(env!("CARGO_PKG_VERSION")));

        // Re-creating the command object for executing another command
        // (as Command object takes ownership of its arguments)
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        // Executing the version command with --v flag
        // and asserting the command runs successfully and outputs the correct version
        cmd.arg("--v")
            .assert()
            .success()
            .stdout(predicates::str::contains(env!("CARGO_PKG_VERSION")));
    }
}
