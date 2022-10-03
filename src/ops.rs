use futures::future::ok;
use tokio::time::error;

use crate::cli::cmd;
use crate::consts;
use std::fmt;
use std::fs::{create_dir_all, File};
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

fn generate_cluster_name() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyz0123456789";
    let rand = random_string::generate(16, charset);
    format!("{}{}", consts::host::CONTAINER_NAME_PREFIX, rand)
}

fn obtain_and_save_cluster_name() -> Result<String, io::Error> {
    let mut file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .append(false)
        .open(consts::host::CLUSTER_NAME_FILE)?;
    
    let mut name = String::new();
    file.read_to_string(&mut name)?;

    name = name.lines().next().unwrap_or("").to_string();

    if name.is_empty() || !name.starts_with(consts::host::CONTAINER_NAME_PREFIX) {
        name = generate_cluster_name();
        file.rewind()?;
        file.set_len(0)?;
        file.write(format!("{}\n", name).as_bytes())?;
        file.flush()?;
    }

    file.sync_data()?;

    Ok(name)
}

fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn create_dir(path: &str) -> bool {
    create_dir_all(path).is_ok()
}

fn report_dependencies() {
    todo!()
}

pub fn provision() {
    if !create_dir(consts::host::STATE_DIR) {
        println!("unable to create {}", consts::host::STATE_DIR);
        return;
    }

    let cluster_name = match obtain_and_save_cluster_name() {
        Ok(name) => name,
        Err(e) => {
            println!("unable to obtaine cluster name due to: {}", e);
            return;
        },
    };

    let cmd = format!("docker ps -q -f name={}", cluster_name);
    let container_exists = match exec(&cmd) {
        Ok(s) => !s.trim().is_empty(),
        Err(e) => {
            println!("unable to execute \"{}\": {}", cmd, e);
            report_dependencies();
            return;
        },
    };

    let cmd = format!("docker ps -q -f name={} -f status=running", cluster_name);
    let container_running = match exec(&cmd) {
        Ok(s) => !s.trim().is_empty(),
        Err(e) => {
            println!("unable to execute \"{}\": {}", cmd, e);
            report_dependencies();
            return;
        },
    };
    
    println!("container_exists: {container_exists}, container_running: {container_running}");

    println!("{}", exec("docker ps -a").unwrap());
}

pub fn remove() {}

pub fn logs() {}

pub fn execute() {}

pub fn copy() {}

pub fn refresh() {}

pub fn run() {}

pub fn shell() {}

pub fn kubectl() {}
