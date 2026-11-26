use colored::Color;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::path::PathBuf;
use std::{env, fs, io};

mod builtins;
use builtins::{albashell, cd, export, type_cmd};

mod parser;
mod prompt;
mod retvals;
mod runner;
use parser::Cmd;

const ERROR: &str = "\x1b[1m[\x1b[31mERROR\x1b[0m\x1b[1m]\x1b[0m";
const WARNING: &str = "\x1b[1m[\x1b[33mWARNING\x1b[0m\x1b[1m]\x1b[0m";
const DEFAULT_COL: Color = Color::Cyan;

fn main() {
    const BUILTINS: &[&str] = &["albashell", "cd", "exit", "export", "type"];

    let mut color: Color = DEFAULT_COL;
    let mut sparse: bool = true;

    let username: String = whoami::username().unwrap_or_else(|_| "<unknown>".to_string());
    let hostname: String = whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string());
    let homedir: PathBuf = dirs::home_dir().unwrap_or(PathBuf::from(
        env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    ));

    let mut rl = DefaultEditor::new().unwrap_or_else(|e| {
        eprintln!("{ERROR}: unable to initialize line editor: {e}");
        std::process::exit(retvals::ERR_UNKNOWN)
    });

    let mut path: std::path::PathBuf = dirs::data_dir().unwrap_or_else(|| {
        eprintln!("{ERROR}: Data directory not found");
        std::process::exit(retvals::ERR_NOTFOUND);
    });
    path.push("albashell");
    match fs::create_dir_all(path.as_path()) {
        Ok(()) => {}
        Err(e) => {
            if e.kind() != io::ErrorKind::AlreadyExists {
                eprintln!("{ERROR}: unable to create the data directory: {e}");
                std::process::exit(retvals::ERR_UNKNOWN);
            }
        }
    }
    path.push("history");
    let _ = rl.load_history(&path);

    let retval: i32 = loop {
        let mut retcode: i32 = retvals::RET_OK;
        let prompt: String = prompt::new(&username, &hostname, retcode, &homedir, color);

        let line: String = match rl.readline(&prompt) {
            Ok(line) => line,
            // Ctrl+C
            Err(ReadlineError::Interrupted) => continue,
            // Ctrl+D
            Err(ReadlineError::Eof) => {
                eprintln!("[albashell] exiting...");
                break retvals::RET_OK;
            }
            Err(e) => {
                eprintln!("{ERROR}: unable to get user input: {e}");
                break retvals::ERR_UNKNOWN;
            }
        };

        let tline: &str = line.trim();
        if tline.is_empty() {
            continue;
        }
        let _ = rl.add_history_entry(tline);

        let cmds: Vec<Cmd> = parser::parse(tline, &homedir);

        for cmd in cmds {
            if !cmd.should_run(retcode) {
                continue;
            }

            retcode = match cmd.executable.as_str() {
                "albashell" => albashell(&cmd.args, &mut color, &mut sparse),
                "cd" => cd(&cmd.args, &homedir),
                "exit" => {
                    let _ = rl.save_history(&path);
                    if cmd.args.is_empty() {
                        std::process::exit(retvals::RET_OK);
                    }
                    std::process::exit(cmd.args[0].parse().unwrap_or(retvals::ERR_UNKNOWN));
                }
                "export" => export(&cmd.args),
                "type" => type_cmd(&cmd.args, BUILTINS),
                _ => runner::run(&cmd),
            };
            if sparse {
                println!()
            }
        }
    };

    let _ = rl.save_history(path.to_str().unwrap_or("~/.local/share/albashell"));

    std::process::exit(retval);
}
