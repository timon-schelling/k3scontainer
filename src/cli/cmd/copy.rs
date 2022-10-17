use crate::cli::Command;

use clap::Args;
#[derive(Args, Clone)]
pub struct Cmd;

impl Command for Cmd {
    fn main(&self) {
        crate::ops::copy();
    }
}
