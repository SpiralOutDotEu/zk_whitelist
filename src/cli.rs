use clap::{Parser, Subcommand};
use fake::{faker::lorem::en::Sentence, Fake};
use std::io;
mod commands;
use crate::utils::{command_runner::RealCommandRunner, filesystem_operations::RealFileSystemOps};
use commands::{all, circuit, compile, movejs, proofs, setup, token, verifier};

/// Represents the command line interface for the Zero Knowledge Whitelist Tool.
/// Deriving `Parser` from clap allows for automatic parsing of command line arguments.
#[derive(Parser)]
#[clap(
    name = "zk_whitelist",
    version = env!("CARGO_PKG_VERSION"),
    author = "Nikos Koumbakis <n.koumbakis@gmail.com>",
    about = "This tool orchestrates the management of an address whitelist using Zero-Knowledge (ZK) proofs.\nSimply input the addresses, and it will generate the corresponding Solidity code.\nIt streamlines the process of maintaining a secure and efficient whitelist for your decentralized application."
)]
pub struct Cli {
    /// The subcommand to be executed, parsed from the command line arguments.
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// Enumerates the available subcommands.
/// Deriving `Subcommand` from clap provides automatic subcommand handling.
#[derive(Subcommand, PartialEq, Debug)]
pub enum SubCommand {
    /// The `circuit` subcommand copies a circuit file template to the current directory.
    Circuit,
    /// The `compile` subcommand compiles the circuit file.
    Compile,
    /// Executes the setup ceremony to generate secure setup
    Setup,
    /// Exports a Solidity verifier
    Verifier,
    /// Moves the contents of `circuit_js` on parent directory
    Movejs,
    /// Generates a sample token solidity contract, to be used together with verifier.
    Token,
    /// Generates proofs using an input file, with a default value of "addresses.txt".
    Proofs(ProofsCommand),
    /// Run all the commands one after the other, {circuit, compile, setup, verifier, movejs, token, proofs} using an input file, with a default value of "addresses.txt"
    All(AllCommand),
}

#[derive(Parser, PartialEq, Debug)]
pub struct ProofsCommand {
    #[clap(long, default_value = "addresses.txt")]
    pub input_file: String,
}

#[derive(Parser, PartialEq, Debug)]
pub struct AllCommand {
    #[clap(long, default_value = "addresses.txt")]
    pub input_file: String,
}

/// The entry point of the application.
/// Parses command line arguments and executes the corresponding subcommand.
pub fn run_cli() -> std::io::Result<()> {
    let args = Cli::parse();
    let runner = RealCommandRunner;
    let file_system_ops = RealFileSystemOps;
    let random_name: String = Sentence(2..3).fake();
    let random_text: String = Sentence(3..4).fake();

    match args.subcmd {
        SubCommand::Circuit => circuit::handle_circuit_subcommand()?,
        SubCommand::Compile => compile::handle_compile_subcommand(&runner)?,
        SubCommand::Setup => setup::handle_setup_subcommand(&runner, random_name, random_text)?,
        SubCommand::Verifier => verifier::handle_verifier_subcommand(&runner)?,
        SubCommand::Movejs => movejs::handle_movejs_subcommand(&file_system_ops)?,
        SubCommand::Proofs(proofs_command) => {
            proofs::handle_proofs_subcommand(&runner, &proofs_command.input_file, &file_system_ops)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        }
        SubCommand::Token => token::handle_token_subcommand()?,
        SubCommand::All(all_command) => {
            all::handle_all_command(
                runner,
                random_name,
                random_text,
                file_system_ops,
                all_command,
            )?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_circuit_subcommand() {
        let args = Cli::parse_from(&["zk_whitelist", "circuit"]);
        assert_eq!(args.subcmd, SubCommand::Circuit);
    }

    #[test]
    fn test_parse_compile_subcommand() {
        let args = Cli::parse_from(&["zk_whitelist", "compile"]);
        assert_eq!(args.subcmd, SubCommand::Compile);
    }

    #[test]
    fn test_parse_setup_subcommand() {
        let args = Cli::parse_from(&["zk_whitelist", "setup"]);
        assert_eq!(args.subcmd, SubCommand::Setup);
    }

    #[test]
    fn test_parse_verifier_subcommand() {
        let args = Cli::parse_from(&["zk_whitelist", "verifier"]);
        assert_eq!(args.subcmd, SubCommand::Verifier);
    }

    #[test]
    fn test_movejs_subcommand() {
        let args = Cli::parse_from(&["zk_whitelist", "movejs"]);
        assert_eq!(args.subcmd, SubCommand::Movejs);
    }

    #[test]
    fn test_parse_proofs_subcommand_with_default_value() {
        let args = Cli::parse_from(&["zk_whitelist", "proofs"]);
        assert_eq!(
            args.subcmd,
            SubCommand::Proofs(ProofsCommand {
                input_file: "addresses.txt".to_string()
            })
        );
    }

    #[test]
    fn test_parse_proofs_subcommand_with_custom_value() {
        let args = Cli::parse_from(&["zk_whitelist", "proofs", "--input-file", "custom.txt"]);
        assert_eq!(
            args.subcmd,
            SubCommand::Proofs(ProofsCommand {
                input_file: "custom.txt".to_string()
            })
        );
    }

    #[test]
    fn test_parse_all_subcommand_with_default_value() {
        let args = Cli::parse_from(&["zk_whitelist", "all"]);
        assert_eq!(
            args.subcmd,
            SubCommand::All(AllCommand {
                input_file: "addresses.txt".to_string()
            })
        );
    }

    #[test]
    fn test_parse_all_subcommand_with_custom_value() {
        let args = Cli::parse_from(&["zk_whitelist", "all", "--input-file", "custom.txt"]);
        assert_eq!(
            args.subcmd,
            SubCommand::All(AllCommand {
                input_file: "custom.txt".to_string()
            })
        );
    }
}
