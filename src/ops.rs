use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum ExecError {
    NoneZeroExitCode {
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
    IoError(io::Error),
}

impl std::error::Error for ExecError {}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecError::NoneZeroExitCode {
                exit_code,
                stdout: _,
                stderr: _,
            } => write!(f, "NoneZeroExitCode Error: {exit_code}"),
            ExecError::IoError(e) => e.fmt(f),
        }
    }
}

fn exec(cmd: &str) -> Result<String, ExecError> {
    let mut iter = cmd.split_whitespace();
    let bin = iter.next().unwrap().to_string();
    let args = iter.map(String::from).collect::<Vec<String>>();
    match Command::new(bin).args(args).output() {
        Ok(out) => {
            if !out.status.success() {
                return Err(ExecError::NoneZeroExitCode {
                    exit_code: out.status.code().unwrap_or(1),
                    stdout: String::from_utf8_lossy(&out.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&out.stderr).to_string(),
                });
            }
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        }
        Err(e) => Err(ExecError::IoError(e)),
    }
}

fn read(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(Path::new(&path))?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn provision() {
    println!("{}", exec("docker ps -a").unwrap())
}

pub fn remove() {}

pub fn logs() {}

pub fn execute() {}

pub fn copy() {}

pub fn refresh() {}

pub fn run() {}

pub fn shell() {}

pub fn kubectl() {}
