use crate::consts;
use std::{fmt, result};
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

fn exec(cmd: &str, input: &str) -> Result<String, Error> {
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
            Err(e) => {
                return Err(Error::CommandIoError {
                    cmd: cmd.to_string(),
                    cause: e,
                })
            }
        };

        let stdin = prosses.stdin.as_mut().unwrap();

        match stdin.write_all(input.as_bytes()) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error::CommandIoError {
                    cmd: cmd.to_string(),
                    cause: e,
                })
            }
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
            Err(e) => {
                return Err(Error::CommandIoError {
                    cmd: cmd.to_string(),
                    cause: e,
                })
            }
        }
    };

    match prosses.wait_with_output() {
        Ok(out) => {
            if !out.status.success() {
                return Err(Error::CommandReturnedNoneZeroExitCodeError {
                    cmd: cmd.to_string(),
                    exit_code: out.status.code().unwrap_or(1),
                    stdout: String::from_utf8_lossy(&out.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&out.stderr).to_string(),
                });
            }
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        }
        Err(e) => {
            return Err(Error::CommandIoError {
                cmd: cmd.to_string(),
                cause: e,
            })
        }
    }
}

fn exec_no_input(cmd: &str) -> Result<String, Error> {
    exec(cmd, "")
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

fn obtain_cluster_name() -> Result<String, Error> {
    fn inner() -> Result<Option<String>, io::Error> {
        let mut file = open_cluster_name_file()?;
        let name = read_cluster_name_from(&mut file)?;

        if name.is_empty() {
            return Ok(None);
        }

        Ok(Some(name))
    }
    match inner() {
        Ok(Some(name)) => Ok(name),
        Ok(None) => Err(Error::ObtaineClusterNameEmptyError),
        Err(e) => Err(Error::ObtaineClusterNameError { cause: e }),
    }
}

fn obtain_or_generate_and_save_cluster_name() -> Result<String, Error> {
    fn inner() -> Result<String, io::Error> {
        let mut file = open_cluster_name_file()?;
        let mut name = read_cluster_name_from(&mut file)?;

        if name.is_empty() {
            name = generate_cluster_name();
            file.rewind()?;
            file.set_len(0)?;
            file.write(format!("{}\n", name).as_bytes())?;
            file.flush()?;
            file.sync_data()?;
        }

        Ok(name)
    }
    match inner() {
        Ok(name) => Ok(name),
        Err(e) => Err(Error::ObtaineOrGenerateAndSaveClusterNameError { cause: e }),
    }
}

fn obtain_and_save_build_input() -> Result<String, Error> {
    fn inner() -> Result<String, io::Error> {
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
    match inner() {
        Ok(content) => Ok(content),
        Err(e) => Err(Error::ObtaineBuildInputError { cause: e }),
    }
}

fn report_dependencies() {
    todo!()
}

#[derive(Debug)]
pub enum Error {
    CreateDirError {
        path: String,
        cause: io::Error,
    },
    ObtaineClusterNameEmptyError,
    ObtaineClusterNameError {
        cause: io::Error,
    },
    ObtaineOrGenerateAndSaveClusterNameError {
        cause: io::Error,
    },
    ObtaineBuildInputError {
        cause: io::Error,
    },
    CommandIoError {
        cmd: String,
        cause: io::Error,
    },
    CommandReturnedNoneZeroExitCodeError {
        cmd: String,
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
    ClusterContaienrNotInRunningStateError {
        state: String,
    },

}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

fn create_dir(path: &str) -> Result<(), Error> {
    match create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::CreateDirError {
            path: path.to_string(),
            cause: e,
        }),
    }
}

pub fn provision() -> Result<(), Error> {
    create_dir(consts::host::STATE_DIR)?;

    let cluster_name = obtain_or_generate_and_save_cluster_name()?;

    let cmd = format!("docker ps -a -q -f name={} -f status=running", cluster_name);
    let out = exec_no_input(&cmd)?;

    let container_running = !out.trim().is_empty();

    if container_running {
        println!("cluster is already up");
        return Ok(());
    }

    let cmd = format!("docker ps -a -q -f name={}", cluster_name);
    let out = exec_no_input(&cmd)?;
    let container_exists = !out.trim().is_empty();

    if container_exists {
        let cmd = format!("docker inspect -f '{{{{.State.Status}}}}' {}", cluster_name);
        let out = exec_no_input(&cmd)?;
        let state = out.trim_end().replace("'", "");
        return Err(Error::ClusterContaienrNotInRunningStateError { state });
    }

    let cmd = format!("docker build -o plain -t {} -", cluster_name);
    let input = obtain_and_save_build_input()?;
    exec(&cmd, &input)?;

    let cmd = format!("docker volume create {}-docker-dir-volume", cluster_name);
    exec_no_input(&cmd)?;

    
    let pwd = std::env::current_dir().unwrap();
    let pwd = pwd.to_str().unwrap();
    let cmd = &[
        "docker run",
        format!("--name {}", cluster_name).as_str(),
        "--privileged",
        "--detach",
        "--restart unless-stopped",
        format!("-v {}:{}:ro", pwd, consts::container::HOST_WORK_DIR_MOUNT).as_str(),
        format!("-v {}-docker-dir-volume:/var/lib/docker", cluster_name).as_str(),
        format!("-v {}/{}:{}", pwd, consts::host::DATA_DIR, consts::container::DATA_DIR).as_str(),
        &cluster_name,
    ]
    .join(" ");
    exec_no_input(&cmd)?;

    Ok(())
}

pub fn remove() -> Result<(), Error> {
    let cluster_name = obtain_cluster_name()?;

    let cmd = format!("docker rm --force {}", cluster_name);
    exec_no_input(&cmd)?;

    let cmd = format!("docker volume rm {}-docker-dir-volume", cluster_name);
    exec_no_input(&cmd)?;

    let cmd = format!("docker image rm {}", cluster_name);
    exec_no_input(&cmd)?;

    Ok(())
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
