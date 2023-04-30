fn main() {
    let mut args = std::env::args();

    match args.len() {
        1 => stellar::repl(),
        2 => stellar::run_file(args.nth(1).unwrap()),
        _ => stellar::print_usage(),
    }
}
