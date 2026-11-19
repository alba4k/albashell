use colored::{Color, Colorize};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output};

mod builtins;
use builtins::*;
mod parser;

const ERROR: &str = "\x1b[1m[\x1b[31mERROR\x1b[0m\x1b[1m]\x1b[0m";

fn main() {
    const BUILTINS: &[&str] = &["echo", "type", "exit", "cd"];

    let username: String = whoami::username().unwrap_or("<unknown>".to_string());
    let hostname: String = whoami::hostname().unwrap_or("<unknown>".to_string());
    let homedir: String = env::var("HOME").unwrap_or("/".to_string());

    const COL: Color = Color::Cyan;
    let mut retcode: i32 = 0;

    loop {
        print_prompt(&username, &hostname, retcode, &homedir, COL);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_else(|_| {
            eprintln!("{ERROR}: unable to get user input");
            std::process::exit(-1)
        });

        if input.eq("\n") {
            continue;
        }

        let words: Vec<&str> = parser::parse(&input);

        retcode = match words[0] {
            "echo" => echo(&words[1..]),
            "cd" => cd(&words[1..], &PathBuf::from(&homedir)),
            "exit" => {
                if words.len() > 1 {
                    std::process::exit(words[1].parse().unwrap_or(-1))
                } else {
                    std::process::exit(0)
                }
            }
            "type" => type_cmd(&words[1..], BUILTINS),
            cmd => exec_cmd(cmd, &words[1..]),
        };
    }
}

fn print_prompt(username: &str, hostname: &str, retcode: i32, homedir: &str, color: Color) {
    let width: usize = term_size::dimensions().unwrap_or((0, 0)).0;
    let cwd: String = env::current_dir()
        .unwrap_or(PathBuf::from("/"))
        .to_string_lossy()
        .replace(&homedir, "~");

    let left_half: String = format!(
        "{}{}{}{}{}{}{}",
        "╭─".black(),
        cwd.color(color).on_black(),
        " | ".on_black(),
        username.color(color).on_black(),
        "@".on_black(),
        hostname.color(color).on_black(),
        "".black()
    );
    let right_half: String = format!(
        "{}{}{}",
        "".black(),
        if retcode == 0 {
            "󰸞".green().on_black()
        } else {
            format!(" {retcode}").red().on_black()
        },
        "".black(),
    );

    println!(
        "{left_half}{}{right_half}",
        "·".repeat(
            width
                .checked_sub(
                    ansi_width::ansi_width(&left_half) + ansi_width::ansi_width(&right_half)
                )
                .unwrap_or(0)
        )
        .black(),
    );

    print!("{}", "╰─ ".black());

    let _ = io::stdout().flush();
}

fn exec_cmd(cmd: &str, args: &[&str]) -> i32 {
    let output: Output = match Command::new(cmd).args(args.iter()).output() {
        Ok(out) => out,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("{ERROR} {cmd}: Command not found")
                }
                _ => eprintln!("{ERROR} Unknown error"),
            };
            return -1;
        }
    };
    print!(
        "{}",
        String::from_utf8(output.stdout).unwrap_or(String::new())
    );

    output.status.code().unwrap_or(-1) // None if signal-terminated
}
