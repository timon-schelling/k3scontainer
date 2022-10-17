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
        let mut prosses = match Command::new(bin)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(r) => r,
            Err(e) => return Err(ExecError::IoError(e)),
        };

        let stdin = prosses.stdin.as_mut().unwrap();

        match stdin.write_all(input.as_bytes()) {
            Ok(r) => r,
            Err(e) => return Err(ExecError::IoError(e)),
        };

        drop(stdin);

        prosses
    } else {
        match Command::new(bin)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
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
            if let ExecError::NoneZeroExitCode {
                exit_code: _,
                stdout,
                stderr,
            } = &e
            {
                println!();
                println!("{}", stdout);
                println!();
                println!("{}", stderr);
            }
            println!();
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

fn open_cluster_name_file() -> Result<File, io::Error> {
    let file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .append(false)
        .open(consts::host::CLUSTER_NAME_FILE)?;
    
    Ok(file)
}

fn read_cluster_name_from(file: &mut File) -> Result<String, io::Error> {
    let mut name = String::new();
    file.read_to_string(&mut name)?;
    name = name.lines().next().unwrap_or("").to_string();
    Ok(name)
}

fn obtain_cluster_name() -> Result<Option<String>, io::Error> {
    let mut file = open_cluster_name_file()?;
    let name = read_cluster_name_from(&mut file)?;

    if name.is_empty() || !name.starts_with(consts::host::CONTAINER_NAME_PREFIX) {
        return Ok(None)
    }

    Ok(Some(name))
}

fn obtain_or_generate_and_save_cluster_name() -> Result<String, io::Error> {
    let mut file = open_cluster_name_file()?;
    let mut name = read_cluster_name_from(&mut file)?;

    if name.is_empty() || !name.starts_with(consts::host::CONTAINER_NAME_PREFIX) {
        name = generate_cluster_name();
        file.rewind()?;
        file.set_len(0)?;
        file.write(format!("{}\n", name).as_bytes())?;
        file.flush()?;
        file.sync_data()?;
    }

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

    if content.is_empty() {
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


#[derive(Debug)]
pub enum Error {
    IoError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exec::NoneZeroExitCode {
                exit_code,
                stdout: _,
                stderr: _,
            } => write!(f, "NoneZeroExitCode Error: {exit_code}"),
            ExecError::IoError(e) => e.fmt(f),
        }
    }
}


pub fn provision() {
    if !create_dir(consts::host::STATE_DIR) {
        println!("unable to create {}", consts::host::STATE_DIR);
        return;
    }

    let cluster_name = match obtain_or_generate_and_save_cluster_name() {
        Ok(name) => name,
        Err(e) => {
            println!("unable to obtaine or generate and save cluster name due to: {}", e);
            return;
        }
    };

    let cmd = format!("docker ps -a -q -f name={} -f status=running", cluster_name);
    let container_running = match exec_no_input_report_error(&cmd) {
        Ok(s) => !s.trim().is_empty(),
        Err(_) => return,
    };

    if container_running {
        println!("cluster is already up");
        return;
    }

    let cmd = format!("docker ps -a -q -f name={}", cluster_name);
    let container_exists = match exec_no_input_report_error(&cmd) {
        Ok(s) => !s.trim().is_empty(),
        Err(_) => return,
    };

    if container_exists {
        let cmd = format!("docker inspect -f '{{{{.State.Status}}}}' {}", cluster_name);
        let container_state = match exec_no_input_report_error(&cmd) {
            Ok(s) => s.trim_end().replace("'", ""),
            Err(_) => return,
        };

        println!(
            "cluster container exits but is not in runnig state: {}",
            container_state
        );
        return;
    }

    let cmd = format!("docker build -o plain -t {} -", cluster_name);
    let input = match obtain_and_save_build_input() {
        Ok(r) => r,
        Err(e) => {
            println!("unable to obtaine build input due to: {}", e);
            return;
        }
    };
    match exec_report_error(&cmd, &input) {
        Ok(_) => {}
        Err(_) => return,
    };

    let cmd = format!("docker volume create {}-docker-dir-volume", cluster_name);
    match exec_no_input_report_error(&cmd) {
        Ok(_) => {}
        Err(_) => return,
    };

    let pwd_buf = std::env::current_dir().unwrap();
    let pwd = pwd_buf.to_str().unwrap();
    let cmd = &[
        "docker run",
        format!("--name {}", cluster_name).as_str(),
        "--privileged",
        "--detach",
        "--restart unless-stopped",
        format!("-v {}:/k3scontainer/pwd:ro", pwd).as_str(),
        format!("-v {}-docker-dir-volume:/var/lib/docker", cluster_name).as_str(),
        format!(
            "-v {}/{}:{}",
            pwd,
            consts::host::DATA_DIR,
            consts::container::DATA_DIR
        )
        .as_str(),
        &cluster_name,
    ]
    .join(" ");
    match exec_no_input_report_error(&cmd) {
        Ok(_) => {}
        Err(_) => return,
    };
}

pub fn remove() {
    let cluster_name = match obtain_cluster_name() {
        Ok(Some(name)) => name,
        Ok(None) => return,
        Err(e) => {
            println!("unable to obtaine cluster name due to: {}", e);
            return;
        }
    };

    let cmd = format!("docker rm --force {}", cluster_name);
    match exec_no_input_report_error(&cmd) {
        Ok(_) => {}
        Err(_) => return,
    };

    let cmd = format!("docker volume rm {}-docker-dir-volume", cluster_name);
    match exec_no_input_report_error(&cmd) {
        Ok(_) => {}
        Err(_) => return,
    };

    let cmd = format!("docker image rm {}", cluster_name);
    match exec_no_input_report_error(&cmd) {
        Ok(_) => {}
        Err(_) => return,
    };
}

pub fn logs() {}

pub fn execute() {}

pub fn copy() {}

pub fn refresh() {}

pub fn run() {}

pub fn shell() {}

pub fn kubectl() {}

pub fn container_entrypoint() {}

pub fn container_refresh() {}
