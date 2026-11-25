use std::io;
use std::process::Command;

use crate::parser::Cmd;
use crate::{ERROR, retvals};

pub fn run(cmd: &Cmd) -> i32 {
    match Command::new(cmd.executable.as_str())
        .args(&cmd.args)
        .status()
    {
        Ok(status) => status.code().unwrap_or(retvals::RET_SIGNAL), // None if signal-terminated
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("{ERROR} {}: Command not found", cmd.executable);
                retvals::ERR_NOTFOUND
            } else {
                eprintln!("{ERROR} Unknown error");
                retvals::ERR_UNKNOWN
            }
        }
    }
}
