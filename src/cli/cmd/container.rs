use crate::cli::Command;

pub mod entrypoint;
pub mod refresh;

use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum Cmd {
    Entrypoint(entrypoint::Cmd),
    Refresh(refresh::Cmd),
}

impl Command for Cmd {
    fn main(&self) {
        match &self {
            Cmd::Entrypoint(cmd) => cmd.main(),
            Cmd::Refresh(cmd) => cmd.main(),
        }
    }
}
