use clap::command;

use crate::consts;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::{self, prelude::*};
use std::process::{Command, Stdio};

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

fn exec(cmd: &str, input: &str) -> Result<String, ExecError> {
    let mut iter = cmd.split_whitespace();
    let bin = iter.next().unwrap().to_string();
    let args = iter.map(String::from).collect::<Vec<String>>();

    let prosses = if !input.is_empty() {
        let mut prosses = match Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(r) => r,
            Err(e) => return Err(ExecError::IoError(e)),
        };

        let stdin = prosses
            .stdin
            .as_mut()
            .expect("stdin was provided for process");

        match stdin.write_all(input.as_bytes()) {
            Ok(r) => r,
            Err(e) => return Err(ExecError::IoError(e)),
        };

        drop(stdin);

        prosses
    } else {
        match Command::new(bin).args(args).spawn() {
            Ok(r) => r,
            Err(e) => return Err(ExecError::IoError(e)),
        }
    };

    match prosses.wait_with_output() {
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

fn exec_no_input(cmd: &str) -> Result<String, ExecError> {
    exec(cmd, "")
}

fn exec_report_error(cmd: &str, input: &str) -> Result<String, ExecError> {
    match exec(cmd, input) {
        Ok(r) => Ok(r),
        Err(e) => {
            println!("unable to execute \"{}\": {}", cmd, e);
            report_dependencies();
            Err(e)
        }
    }
}

fn exec_no_input_report_error(cmd: &str) -> Result<String, ExecError> {
    exec_report_error(cmd, "")
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

fn obtain_and_save_build_input() -> Result<String, io::Error> {
    let mut file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .append(false)
        .open(consts::host::CONTAINER_BUILD_FILE)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    content = content
        .lines()
        .next()
        .unwrap_or("")
        .to_string();

    if content.is_empty() || !content.starts_with(consts::host::CONTAINER_NAME_PREFIX) {
        content = consts::host::CONTAINER_BUILD_FILE_CONTENT.to_string();
        file.rewind()?;
        file.set_len(0)?;
        file.write(content.as_bytes())?;
        file.flush()?;
    }

    file.sync_data()?;

    Ok(content)
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
        }
    };

    let cmd = format!("docker ps -q -f name={}", cluster_name);
    let container_exists = match exec_no_input_report_error(&cmd) {
        Ok(s) => !s.trim().is_empty(),
        Err(_) => return,
    };

    let cmd = format!("docker ps -q -f name={} -f status=running", cluster_name);
    let container_running = match exec_no_input_report_error(&cmd) {
        Ok(s) => !s.trim().is_empty(),
        Err(_) => return,
    };

    let cmd = format!("docker build -o plain -t {} -", cluster_name);
    let input = match obtain_and_save_build_input() {
        Ok(r) => r,
        Err(e) => {
            println!("unable to obtaine build input due to: {}", e);
            return;
        },
    };
    let image_build = match exec_report_error(&cmd, &input) {
        Ok(s) => true,
        Err(_) => return,
    };

    println!("container_exists: {container_exists}, container_running: {container_running}");

    println!("{}", exec_no_input("docker ps -a").unwrap());
}

pub fn remove() {}

pub fn logs() {}

pub fn execute() {}

pub fn copy() {}

pub fn refresh() {}

pub fn run() {}

pub fn shell() {}

pub fn kubectl() {}
