use k3scontainer::cli;

fn main() {
    cli::main();
}

// use std::str;

// use futures::StreamExt;

// use docker_api::api::container::opts::ContainerCreateOpts;
// use docker_api::api::image::opts::PullOpts;
// use docker_api::{conn::TtyChunk, Docker};

// #[cfg(unix)]
// pub fn new_docker() -> Docker {
//     Docker::new("unix:///var/run/docker.sock").unwrap()
// }

// #[cfg(not(unix))]
// pub fn new_docker() -> Docker {
//     Docker::new("tcp://localhost:2375").unwrap()
// }

// pub fn print_chunk(chunk: TtyChunk) {
//     match chunk {
//         TtyChunk::StdOut(bytes) => {
//             println!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
//         }
//         TtyChunk::StdErr(bytes) => {
//             eprintln!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
//         }
//         TtyChunk::StdIn(_) => unreachable!()
//     }
// }

// #[tokio::main]
// async fn main() {
//     let docker = new_docker();

//     let images = docker.images();

//     let mut pull_result_stream = images.pull(
//         &PullOpts::builder()
//             .image("docker.io/library/ubuntu:latest")
//             .build(),
//     );
//     while let Some(pull_result) = pull_result_stream.next().await {
//         match pull_result {
//             Ok(_) => {}
//             Err(e) => eprintln!("Error: {}", e),
//         }
//     }

//     let opts = ContainerCreateOpts::builder("docker.io/library/ubuntu:latest")
//         .auto_remove(true)
//         .cmd(vec!["echo", "test"])
//         .build();

//     let container = match docker.containers().create(&opts).await {
//         Ok(container) => container,
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             return;
//         }
//     };

//     match container.start().await {
//         Ok(_) => {}
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             return;
//         }
//     };
// }




// mod cli;
// use cli::cli;

// fn push_args() -> Vec<clap::Arg<'static>> {
//     vec![arg!(-m --message <MESSAGE>).required(false)]
// }
