#[cfg(test)]
mod tests {
    // use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_version_command() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("--version")
            .assert()
            .success()
            .stdout(predicates::str::contains(env!("CARGO_PKG_VERSION")));

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("-V")
            .assert()
            .success()
            .stdout(predicates::str::contains(env!("CARGO_PKG_VERSION")));
    }
}
