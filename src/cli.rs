
pub trait Command {
    fn main(&self);
}

pub mod cmd;

use clap::Parser;

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: cmd::Cmd,
}

pub fn main() {
    let cli = Cli::parse();
    cli.cmd.main();
}
