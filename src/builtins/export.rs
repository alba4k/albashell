use crate::ERROR;
use crate::retvals;

pub fn export(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("{ERROR}: Usage: export [VARIABLE] [VALUE]");
        return retvals::ERR_USAGE;
    }

    if args.len() == 1 && args[0].contains('=') {
        let (key, val) = args[0].split_once('=').unwrap_or(("", ""));

        // undefined multithreaded r/w
        unsafe {
            std::env::set_var(key, val);
        }

        return retvals::RET_OK;
    }

    if args.len() != 2 {
        eprintln!("{ERROR}: Usage: export [VARIABLE] [VALUE]");
        return retvals::ERR_USAGE;
    }

    // undefined multithreaded r/w
    unsafe {
        std::env::set_var(&args[0], &args[1]);
    }
    return retvals::RET_OK;
}
