use crate::cli::Command;

#[derive(Clone)]
pub struct Cmd {
    pub args: Vec<String>
}

impl Command for Cmd {
    fn main(&self) {
        todo!()
    }
}
