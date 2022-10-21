use crate::cli::Command;

#[derive(Clone)]
pub struct Cmd {
    args: Vec<String>
}

impl Command for Cmd {
    fn main(&self) {
        todo!()
    }
}
