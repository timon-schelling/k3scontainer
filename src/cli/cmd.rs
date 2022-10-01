use crate::cli::Command;

use clap::Subcommand;

pub mod container;
pub mod copy;
pub mod execute;
pub mod kubectl;
pub mod logs;
pub mod provision;
pub mod refresh;
pub mod remove;
pub mod run;
pub mod shell;

#[derive(Subcommand)]
pub enum Cmd {
    #[command(hide(true))]
    Container {
        #[command(subcommand)]
        cmd: container::Cmd,
    },

    #[command(alias("pv"))]
    Provision(provision::Cmd),

    #[command(alias("rm"))]
    Remove(remove::Cmd),

    #[command(alias("l"))]
    Logs(logs::Cmd),

    #[command(aliases(["exec", "ex"]))]
    Execute(execute::Cmd),

    #[command(alias("cp"))]
    Copy(copy::Cmd),

    #[command(alias("rf"))]
    Refresh(refresh::Cmd),

    #[command(alias("r"))]
    Run(run::Cmd),

    #[command(alias("sh"))]
    Shell(shell::Cmd),

    #[command(alias("k"))]
    Kubectl(kubectl::Cmd),
}

impl Command for Cmd {
    fn main(&self) {
        match &self {
            Cmd::Container { cmd } => cmd.main(),
            Cmd::Provision(cmd) => cmd.main(),
            Cmd::Remove(cmd) => cmd.main(),
            Cmd::Logs(cmd) => cmd.main(),
            Cmd::Execute(cmd) => cmd.main(),
            Cmd::Copy(cmd) => cmd.main(),
            Cmd::Refresh(cmd) => cmd.main(),
            Cmd::Run(cmd) => cmd.main(),
            Cmd::Shell(cmd) => cmd.main(),
            Cmd::Kubectl(cmd) => cmd.main(),
        }
    }
}
