pub fn type_cmd(args: &[&str], builtins: &[&str]) -> i32 {
    if args.len() != 1 {
        eprintln!("Usage: type [command]");
        return -1;
    }
    println!(
        "{} is {}a builtin",
        args[0],
        if builtins.contains(&args[0]) {
            ""
        } else {
            "not "
        }
    );
    0
}
