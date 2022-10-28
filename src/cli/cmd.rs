use crate::cli::Command;

use clap::Parser;

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

#[derive(Parser)]
pub enum Cmd {
    #[command(hide(true))]
    Container {
        #[command(subcommand)]
        cmd: container::Cmd,
    },

    #[command(alias("pv"))]
    Provision,

    #[command(alias("rm"))]
    Remove,

    #[command(alias("l"))]
    Logs,

    #[command(aliases(["exec", "ex"]))]
    Execute {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    #[command(alias("cp"))]
    Copy,

    #[command(alias("rf"))]
    Refresh,

    #[command(alias("r"))]
    Run,

    #[command(alias("sh"))]
    Shell,

    #[command(alias("k"))]
    Kubectl,
}

impl Command for Cmd {
    fn main(&self) {
        match &self {
            Cmd::Container { cmd } => cmd.main(),
            Cmd::Provision => provision::Cmd.main(),
            Cmd::Remove => remove::Cmd.main(),
            Cmd::Logs => logs::Cmd.main(),
            Cmd::Execute { args } => execute::Cmd { args: args.clone() }.main(),
            Cmd::Copy => copy::Cmd.main(),
            Cmd::Refresh => refresh::Cmd.main(),
            Cmd::Run => run::Cmd.main(),
            Cmd::Shell => shell::Cmd.main(),
            Cmd::Kubectl => kubectl::Cmd.main(),
        }
    }
}
