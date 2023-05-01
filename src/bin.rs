use stellar::Stellar;

fn main() {
    let mut args = std::env::args();
    let mut stellar = Stellar::new();

    match args.len() {
        1 => stellar.repl(),
        2 => stellar.run_file(args.nth(1).unwrap()),
        _ => stellar::print_usage(),
    }
}
