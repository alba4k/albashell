use std::io;
use std::process::Command;

use crate::ERROR;
use crate::parser::Cmd;

pub fn run(cmd: Cmd) -> i32 {
    match Command::new(cmd.executable.as_str())
        .args(&cmd.args)
        .status()
    {
        Ok(status) => status.code().unwrap_or(-1), // None if signal-terminated
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("{ERROR} {}: Command not found", cmd.executable);
                }
                _ => eprintln!("{ERROR} Unknown error"),
            }
            -1
        }
    }
}
