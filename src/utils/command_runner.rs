// src/utils/command_runner.rs

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

#[cfg(test)]
pub struct MockCommandRunner;

#[cfg(test)]
impl CommandRunner for MockCommandRunner {
    fn run(&self, _command: &str, _args: &[&str]) -> Result<(), String> {
        Ok(())
    }
}
