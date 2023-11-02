use crate::{utils::{command_runner::RealCommandRunner, filesystem_operations::RealFileSystemOps}, cli::AllCommand};
use std::io;

use super::{circuit, compile, setup, verifier, movejs, proofs};

pub fn handle_all_command(runner: RealCommandRunner, random_name: String, random_text: String, file_system_ops: RealFileSystemOps, all_command: AllCommand) -> Result<(), io::Error> {
    circuit::handle_circuit_subcommand()?;
    compile::handle_compile_subcommand(&runner)?;
    setup::handle_setup_subcommand(&runner, random_name.clone(), random_text.clone())?;
    verifier::handle_verifier_subcommand(&runner)?;
    movejs::handle_movejs_subcommand(&file_system_ops)?;
    proofs::handle_proofs_subcommand(&runner, &all_command.input_file, &file_system_ops)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}