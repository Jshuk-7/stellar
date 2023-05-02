use stellar::Stellar;

fn main() {
    let mut args = std::env::args();
    let mut properties = stellar::lang::InterpreterProperties::default();

    match args.len() {
        1 => {
            properties.mode = stellar::lang::InterpreterMode::Repl;
            let mut stellar = Stellar::new(properties);
            stellar.repl();
        }
        2 => {
            properties.mode = stellar::lang::InterpreterMode::Script;
            let mut stellar = Stellar::new(properties);
            let path = args.nth(1).unwrap();
            stellar.run_file(path);
        }
        _ => stellar::print_usage(),
    }
}
