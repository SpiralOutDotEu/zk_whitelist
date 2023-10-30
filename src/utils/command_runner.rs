use std::io;

pub trait CommandRunner {
    fn run(&self, command: &str, args: &[&str]) -> Result<(), String>;
}

pub struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, command: &str, args: &[&str]) -> Result<(), String> {
        let output = std::process::Command::new(command)
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(format!("Command failed: {:?}", output));
        }

        Ok(())
    }
}

/// Command runner variant for `snarkjs` commands
pub fn run_snarkjs_command<R: CommandRunner>(runner: &R, args: &[&str]) -> io::Result<()> {
    runner
        .run("snarkjs", args)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

#[cfg(test)]
pub struct MockCommandRunner;

#[cfg(test)]
impl CommandRunner for MockCommandRunner {
    fn run(&self, _command: &str, _args: &[&str]) -> Result<(), String> {
        Ok(())
    }
}
