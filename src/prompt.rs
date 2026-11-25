use colored::{Color, Colorize};
use std::env;

use std::path::PathBuf;

pub fn new(username: &str, hostname: &str, retcode: i32, homedir: &str, color: Color) -> String {
    let width: usize = term_size::dimensions().unwrap_or((0, 0)).0;
    let cwd: String = env::current_dir()
        .unwrap_or(PathBuf::from("/"))
        .to_string_lossy()
        .replace(homedir, "~");

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

    format!(
        "{left_half}{}{right_half}\n{}",
        "·".repeat(width.saturating_sub(
            ansi_width::ansi_width(&left_half) + ansi_width::ansi_width(&right_half)
        ))
        .black(),
        "╰─ ".black()
    )
}
