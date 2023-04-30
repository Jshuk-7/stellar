use std::{collections::HashMap, io::Write};

pub mod lang;

pub fn run(source: String) {
    let mut lexer = lang::Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = lang::Parser::new(tokens);

    let expr = parser.parse();
    let interpreter = lang::Interpreter::new();

    let literal = interpreter.evaluate(expr);
    println!("{literal:?}");
}

pub fn run_file(path: String) {
    if let Ok(source) = std::fs::read_to_string(path) {
        crate::run(source);
    }
}

pub fn repl() {
    let mut line = String::new();

    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();

        crate::run(line.clone());
        line.clear();
    }
}

pub fn make_keywords() -> HashMap<String, lang::TokenType> {
    let mut keywords = HashMap::new();

    type TT = lang::TokenType;
    keywords.insert("if".to_string(), TT::If);
    keywords.insert("let".to_string(), TT::Let);
    keywords.insert("struct".to_string(), TT::Struct);
    keywords.insert("self".to_string(), TT::SSelf);
    keywords.insert("while".to_string(), TT::While);
    keywords.insert("for".to_string(), TT::For);
    keywords.insert("return".to_string(), TT::Return);
    keywords.insert("fun".to_string(), TT::Fun);
    keywords.insert("true".to_string(), TT::True);
    keywords.insert("false".to_string(), TT::False);
    keywords.insert("null".to_string(), TT::Null);

    keywords
}

pub fn error(line: u32, msg: String) {
    println!("[Line: {line}] Error: {msg}")
}

pub fn print_usage() {
    println!(
        "Usage: stellar <script>
Args:
    script: source filepath
    (If no args are provided the interactive repl will start)"
    );
}
