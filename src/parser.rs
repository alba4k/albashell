pub fn parse(input: &String) -> Vec<&str> {
    input
        .strip_suffix('\n')
        .unwrap_or_else(|| {
            // Ctrl+D
            println!();
            std::process::exit(0)
        })
        .split(' ')
        .collect()
}
