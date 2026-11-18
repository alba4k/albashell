use colored::{Color, ColoredString, Colorize};
use std::io;
use std::io::Write;
use std::process::{Command, Output, exit};

const ERROR: &str = "\x1b[1m[\x1b[31mERROR\x1b[0m\x1b[1m]\x1b[0m";

fn main() {
    let builtins: Vec<&str> = vec!["echo", "type", "exit"];

    let username: String = whoami::username().unwrap_or("<unknown>".to_string());
    let hostname: String = whoami::hostname().unwrap_or("<unknown>".to_string());

    const COL: Color = Color::Cyan;
    let mut success_indicator: ColoredString = ">".white();
    let mut retcode: i32 = 0;
    let prompt: String = format!("{}@{}", username.color(COL), hostname.color(COL));
    let char: &str = if username.eq("root") { "#" } else { "$" };

    loop {
        print!(
            "{} {} {} {} ",
            if retcode != 0 {
                format!("({retcode})")
            } else {
                String::new()
            },
            success_indicator,
            prompt,
            char
        );
        let _ = io::stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap_or_else(|_| {
            println!("{ERROR}: unable to get user input");
            std::process::exit(-1)
        });

        if input.eq("\n") {
            continue;
        }

        let words: Vec<&str> = input
            .strip_suffix('\n')
            .unwrap_or_else(|| {
                // Ctrl+D
                println!();
                std::process::exit(0)
            })
            .split(' ')
            .collect();

        match words[0] {
            "echo" => {
                println!("{}", words[1..].join(" "));
            }
            "exit" => {
                if words.len() > 1 {
                    std::process::exit(words[1].parse().unwrap_or(-1))
                } else {
                    std::process::exit(0)
                }
            }
            "type" => {
                if words.len() != 2 {
                    success_indicator = ">".red();
                    eprintln!("Usage: type command");
                    continue;
                }
                println!(
                    "{} is {}a builtin",
                    words[1],
                    if builtins.contains(&words[1]) {
                        ""
                    } else {
                        "not "
                    }
                )
            }
            cmd => {
                let output: Output = match Command::new(cmd).args(words[1..].iter()).output() {
                    Ok(out) => out,
                    Err(e) => {
                        success_indicator = ">".bright_red();
                        match e.kind() {
                            std::io::ErrorKind::NotFound => {
                                println!("{ERROR} 404: Command not found :)")
                            }
                            _ => println!("{ERROR} Unknown error :("),
                        };
                        continue;
                    }
                };
                print!(
                    "{}",
                    String::from_utf8(output.stdout).unwrap_or(String::new())
                );

                retcode = output.status.code().unwrap_or(-1); // None if signal-terminated
                match retcode {
                    0 => success_indicator = ">".green(),
                    _ => success_indicator = ">".red(),
                }
            }
        }
    }
}
