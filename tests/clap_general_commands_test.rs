/// Module for CLI-related tests.
///
/// This module contains tests that verify the behavior of the command-line
/// interface, including argument parsing and error handling.
#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    /// Tests the output of the `--help` flag.
    ///
    /// This test verifies that the `--help` flag produces the expected output,
    /// including a portion of the about message and the names of the available
    /// subcommands.
    #[test]
    fn test_help_flag() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("--help")
            .assert()
            .success()
            .stdout(contains(
                "orchestrates the management of an address whitelist",
            ))
            .stdout(contains("circuit"))
            .stdout(contains("compile"));
    }

    /// Tests the error message for an unrecognized subcommand.
    ///
    /// This test verifies that using an unrecognized subcommand results in an
    /// error message indicating that the subcommand is not recognized.
    #[test]
    fn test_unrecognized_subcommand() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("unrecognized")
            .assert()
            .failure()
            .stderr(contains("error: unrecognized subcommand 'unrecognized'"));
    }
}
