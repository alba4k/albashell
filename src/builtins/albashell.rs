use colored::Color;

use crate::DEFAULT_COL;
use crate::ERROR;
use crate::parser::Cmd;
use crate::retvals;
use crate::runner::run;

pub fn albashell(args: &[String], color: &mut Color) -> i32 {
    if args.is_empty() || (!args[0].eq("conf") && !args[0].eq("config")) {
        return run(&Cmd {
            executable: "albashell".to_string(),
            args: args.to_vec(),
            _background: false,
            cond: None,
        });
    }

    if args.len() < 2 {
        eprintln!("{ERROR}: Usage: albashell conf [OPTIONS]");
        return retvals::ERR_USAGE;
    }

    match args[1].as_str() {
        "col" | "color" => {
            if args.len() < 3 {
                eprintln!("{ERROR}: Not enough arguments for albashell conf col");
                return -1;
            }

            if args[2].eq("default") || args[2].eq("reset") {
                *color = DEFAULT_COL;
                retvals::RET_OK
            } else if let Ok(c) = args[2].parse::<Color>() {
                *color = c;
                retvals::RET_OK
            } else {
                eprintln!("{ERROR}: Unrecognized color {}", args[2]);
                retvals::ERR_USAGE
            }
        }
        _ => {
            eprintln!("{ERROR}: Unrecognized option {}", args[1]);
            retvals::ERR_USAGE
        }
    }
}
