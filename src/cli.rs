// src/cli/cli.rs

use clap::{Parser, Subcommand};
mod commands;
use crate::utils::command_runner::RealCommandRunner;
use commands::{circuit, compile, setup};

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
}

/// The entry point of the application.
/// Parses command line arguments and executes the corresponding subcommand.
pub fn run_cli() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.subcmd {
        SubCommand::Circuit => circuit::handle_circuit_subcommand()?,
        SubCommand::Compile => {
            let runner = RealCommandRunner;
            compile::handle_compile_subcommand(runner)?
        }
        SubCommand::Setup => setup::handle_setup_subcommand()?,
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
}
