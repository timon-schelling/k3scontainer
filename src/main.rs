use std::str;

use futures::StreamExt;

use docker_api::api::container::opts::ContainerCreateOpts;
use docker_api::api::image::opts::PullOpts;
use docker_api::{conn::TtyChunk, Docker};

#[cfg(unix)]
pub fn new_docker() -> Docker {
    Docker::new("unix:///var/run/docker.sock").unwrap()
}

#[cfg(not(unix))]
pub fn new_docker() -> Docker {
    Docker::new("tcp://localhost:2375").unwrap()
}

pub fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => {
            println!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdErr(bytes) => {
            eprintln!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdIn(_) => unreachable!()
    }
}

#[tokio::main]
async fn main() {
    let docker = new_docker();

    let images = docker.images();

    let mut pull_result_stream = images.pull(
        &PullOpts::builder()
            .image("docker.io/library/ubuntu:latest")
            .build(),
    );
    while let Some(pull_result) = pull_result_stream.next().await {
        match pull_result {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    let opts = ContainerCreateOpts::builder("docker.io/library/ubuntu:latest")
        .auto_remove(true)
        .cmd(vec!["echo", "test"])
        .build();

    let container = match docker.containers().create(&opts).await {
        Ok(container) => container,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    match container.start().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
}

// use clap::{arg, Command};
// use std::ffi::OsString;
// use std::path::PathBuf;

// fn cli() -> Command<'static> {
//     Command::new("k3scontainer")
//         .about("run k3s cluster in a container")
//         .subcommand_required(true)
//         .arg_required_else_help(true)
//         .allow_external_subcommands(true)
//         .subcommand(
//             Command::new("provision")
//                 .about("provision new cluster")
//                 .arg(arg!(<ARGS> "arguments passed to docker for container startup"))
//                 .arg_required_else_help(true),
//         )
//         .subcommand(
//             Command::new("push")
//                 .about("pushes things")
//                 .arg(arg!(<ARGS> "arguments passed to docker for container startup")),
//         )
//         .subcommand(
//             Command::new("add")
//                 .about("adds things")
//                 .arg_required_else_help(true)
//                 .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
//         )
//         .subcommand(
//             Command::new("stash")
//                 .args_conflicts_with_subcommands(true)
//                 .args(push_args())
//                 .subcommand(Command::new("push").args(push_args()))
//                 .subcommand(Command::new("pop").arg(arg!([STASH])))
//                 .subcommand(Command::new("apply").arg(arg!([STASH]))),
//         )
// }

// fn push_args() -> Vec<clap::Arg<'static>> {
//     vec![arg!(-m --message <MESSAGE>).required(false)]
// }

// pub fn run(){
//     let matches = cli().get_matches();

//     match matches.subcommand() {
//         Some(("clone", sub_matches)) => {
//             println!(
//                 "Cloning {}",
//                 sub_matches.get_one::<String>("REMOTE").expect("required")
//             );
//         }
//         Some(("push", sub_matches)) => {
//             println!(
//                 "Pushing to {}",
//                 sub_matches.get_one::<String>("REMOTE").expect("required")
//             );
//         }
//         Some(("add", sub_matches)) => {
//             let paths = sub_matches
//                 .get_many::<PathBuf>("PATH")
//                 .into_iter()
//                 .flatten()
//                 .collect::<Vec<_>>();
//             println!("Adding {:?}", paths);
//         }
//         Some(("stash", sub_matches)) => {
//             let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
//             match stash_command {
//                 ("apply", sub_matches) => {
//                     let stash = sub_matches.get_one::<String>("STASH");
//                     println!("Applying {:?}", stash);
//                 }
//                 ("pop", sub_matches) => {
//                     let stash = sub_matches.get_one::<String>("STASH");
//                     println!("Popping {:?}", stash);
//                 }
//                 ("push", sub_matches) => {
//                     let message = sub_matches.get_one::<String>("message");
//                     println!("Pushing {:?}", message);
//                 }
//                 (name, _) => {
//                     unreachable!("Unsupported subcommand `{}`", name)
//                 }
//             }
//         }
//         Some((ext, sub_matches)) => {
//             let args = sub_matches
//                 .get_many::<OsString>("")
//                 .into_iter()
//                 .flatten()
//                 .collect::<Vec<_>>();
//             println!("Calling out to {:?} with {:?}", ext, args);
//         }
//         _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
//     }

//     // Continued program logic goes here...
// }
