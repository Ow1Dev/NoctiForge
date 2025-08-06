use anyhow::Result;
use serde::Deserialize;
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};
use std::process::Stdio;

#[derive(Debug, Deserialize)]
pub struct CustomRun {
    command: String,
    output: String,
}

pub async fn process(cfg: CustomRun, working_dir: &str) -> Result<()> {
    println!("Running command: \"{}\", on path: \"{}\"", cfg.command, working_dir); 

    let mut args = cfg.command.split_whitespace();
    let command = args.next().expect("there must be a command");

    let mut cmd = Command::new(command);
    cmd.args(args).current_dir(working_dir);
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()
        .expect("failed to spawn command");

    let stdout = child.stdout.take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        println!("{}", line);
    }

    let status = child.wait().await
        .expect("child process encountered an error");

    println!("child status was: {}", status);

    Ok(())
}

