use std::env::set_current_dir;
use std::path::{Path, PathBuf};

pub fn cd(args: &[String], homedir: &PathBuf) -> i32 {
    if args.len() > 1 {
        eprintln!("Usage: cd [directory]");
        return -1;
    }
    let newcwd: &Path = if args.is_empty() {
        Path::new(&homedir)
    } else {
        Path::new(args[0].as_str())
    };
    match set_current_dir(newcwd) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}
