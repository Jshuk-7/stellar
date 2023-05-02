use std::io::Write;

pub mod lang;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InterpreterMode {
    Repl,
    Script,
}

pub struct InterpreterProperties {
    mode: InterpreterMode,
}

pub struct Stellar {
    interpreter: lang::Interpreter,
    settings: InterpreterProperties,
}

impl Stellar {
    pub fn new(properties: InterpreterProperties) -> Self {
        Self {
            interpreter: lang::Interpreter::new(properties),
        }
    }

    pub fn run(&mut self, source: String) {
        let mut lexer = lang::Lexer::new(source);
        let tokens = lexer.scan_tokens();

        if let lang::TokenType::Eof = tokens.first().unwrap().ty {
            return;
        }

        if self.handle_error_runtime() {
            return;
        }

        let mut parser = lang::Parser::new(tokens);
        let statements = parser.parse();

        if self.handle_error_runtime() {
            return;
        }

        self.interpreter.interpret(&statements);
    }

    pub fn run_file(&mut self, path: String) {
        if let Ok(source) = std::fs::read_to_string(path) {
            self.run(source);
        }
    }

    pub fn repl(&mut self) {
        crate::print_welcome_msg();

        loop {
            print!(">> ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();

            self.run(line);

            if crate::error_found() {
                crate::set_error_found(false);
            }
        }
    }

    pub fn handle_error_runtime(&self) -> bool {
        if crate::error_found() {
            crate::set_error_found(false);
            return true;
        }

        false
    }
}

mod internal {
    pub static mut ERROR_FOUND: bool = false;
}

pub fn error(line: u32, msg: String) {
    println!("[Line: {line}] Error: {msg}")
}

pub fn error_found() -> bool {
    unsafe { internal::ERROR_FOUND }
}

pub fn set_error_found(err: bool) {
    unsafe {
        internal::ERROR_FOUND = err;
    }
}

pub fn print_literal(literal: lang::Literal) {
    match literal {
        lang::Literal::Number(x) => println!("{x}"),
        lang::Literal::String(x) => println!("{x}"),
        lang::Literal::Bool(x) => println!("{x}"),
        lang::Literal::Char(x) => println!("{x}"),
        lang::Literal::Null => println!("null"),
    }
}

pub fn print_welcome_msg() {
    println!(
        "Welcome to Stellar {VERSION}, running {} on platform {}",
        std::env::consts::ARCH,
        std::env::consts::OS
    );
}

pub fn print_usage() {
    println!("Usage: stellar <script>");
    println!("Args:");
    println!("\tscript: source filepath");
    println!();
    println!("(Hint: Run Stellar with no args to start the interactive REPL)");
}
