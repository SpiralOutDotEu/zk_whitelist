// #[cfg(test)]
// mod tests {
//     use assert_cmd::Command;
//     use std::path::Path;

//     #[test]
//     fn test_setup_command() {
//         // Arrange: Run required preceding commands
//         let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
//         cmd.arg("circuit").assert().success();
//         cmd.arg("compile").assert().success();

//         // Act: Run the setup command
//         cmd.arg("setup").assert().success();

//         // Assert: Check that the required output files are created
//         assert!(Path::new("pot12_0000.ptau").exists());
//         assert!(Path::new("pot12_0001.ptau").exists());
//         assert!(Path::new("pot12_final.ptau").exists());
//         assert!(Path::new("circuit_0000.zkey").exists());
//         assert!(Path::new("circuit_0001.zkey").exists());
//         assert!(Path::new("verification_key.json").exists());
//     }
// }
