use colored::Color;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::env;
use std::path::PathBuf;

mod builtins;
use builtins::{albashell, cd, type_cmd};

mod parser;
mod prompt;
mod runner;
use parser::{Cmd, Condition};

const ERROR: &str = "\x1b[1m[\x1b[31mERROR\x1b[0m\x1b[1m]\x1b[0m";
const DEFAULT_COL: Color = Color::Cyan;

fn main() {
    const BUILTINS: &[&str] = &["albashell", "cd", "exit", "type"];

    let mut color: Color = DEFAULT_COL;

    let username: String = whoami::username().unwrap_or("<unknown>".to_string());
    let hostname: String = whoami::hostname().unwrap_or("<unknown>".to_string());
    let homedir: String = env::var("HOME").unwrap_or("/".to_string());

    let mut retcode: i32 = 0;

    let mut rl = DefaultEditor::new().unwrap_or_else(|e| {
        eprintln!("{ERROR}: unable to initialize line editor: {e}");
        std::process::exit(-1)
    });

    let mut path: std::path::PathBuf = dirs::data_dir().unwrap_or_else(|| {
        eprintln!("{ERROR}: Data directory not found");
        std::process::exit(1);
    });
    path.push("albashell/history");
    let _ = rl.load_history(
        path.to_str()
            .unwrap_or(format!("{homedir}/.local/share/albashell").as_str()),
    );

    retcode = loop {
        let prompt: String = prompt::new(&username, &hostname, retcode, &homedir, color);

        let line: String = match rl.readline(&prompt) {
            Ok(line) => line,
            // Ctrl+C
            Err(ReadlineError::Interrupted) => continue,
            // Ctrl+D
            Err(ReadlineError::Eof) => {
                println!("[albashell] exiting...");
                break 0;
            }
            Err(e) => {
                eprintln!("{ERROR}: unable to get user input: {e}");
                break -1;
            }
        };

        if line.trim().is_empty() {
            continue;
        }
        let _ = rl.add_history_entry(line.as_str());

        let cmds: Vec<Cmd> = parser::parse(&line);

        for cmd in cmds {
            match (&cmd.cond, retcode) {
                (Some(Condition::Success), 0) | (None, _) => {}
                (Some(Condition::Success), _) | (Some(Condition::Failure), 0) => continue,
                (Some(Condition::Failure), _) => {}
            }

            retcode = match cmd.executable.as_str() {
                "albashell" => albashell(cmd.args, &mut color),
                "cd" => cd(cmd.args, &PathBuf::from(&homedir)),
                "exit" => {
                    let _ = rl.save_history(path.to_str().unwrap_or("~/.local/share/albashell"));
                    if cmd.args.is_empty() {
                        std::process::exit(0);
                    }
                    std::process::exit(cmd.args[0].parse().unwrap_or(-1));
                }
                "type" => type_cmd(cmd.args, BUILTINS),

                _ => runner::run(cmd),
            };
        }
    };

    let _ = rl.save_history(path.to_str().unwrap_or("~/.local/share/albashell"));

    std::process::exit(retcode);
}
