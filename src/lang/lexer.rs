use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LCurly,
    RCurly,
    LParen,
    RParen,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Eof,

    // Comparison/Equality
    And,
    Or,
    Bang,
    Eq,
    EqEq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,

    // Operators
    Plus,
    PlusEq,
    Minus,
    MinusEq,
    Star,
    StarEq,
    Slash,
    SlashEq,

    // Keywords
    If,
    Else,
    Let,
    Struct,
    SSelf,
    While,
    For,
    Return,
    Fun,
    True,
    False,
    Null,

    // Literals
    Ident,
    Number,
    String,
    Char,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub line: u32,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, line: u32) -> Self {
        Self { ty, lexeme, line }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("[{:?} {}]", self.ty, self.lexeme)
    }
}

pub struct Lexer {
    source: String,
    chars: Vec<char>,
    cursor: usize,
    start: usize,
    line: u32,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.clone(),
            chars: source.chars().collect(),
            cursor: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
            keywords: vec![
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("and", TokenType::And),
                ("or", TokenType::Or),
                ("let", TokenType::Let),
                ("struct", TokenType::Struct),
                ("self", TokenType::SSelf),
                ("while", TokenType::While),
                ("for", TokenType::For),
                ("return", TokenType::Return),
                ("fun", TokenType::Fun),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("null", TokenType::Null),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        loop {
            if self.is_at_end() {
                break;
            }

            self.start = self.cursor;
            self.scan_token();
        }

        let eof = Token::new(TokenType::Eof, "".to_string(), self.line);
        self.tokens.push(eof);

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '{' => self.add_token(TokenType::LCurly),
            '}' => self.add_token(TokenType::RCurly),
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ':' => self.add_token(TokenType::Colon),
            ';' => self.add_token(TokenType::Semicolon),
            '+' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::PlusEq)
                } else {
                    self.add_token(TokenType::Plus)
                }
            }
            '-' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::MinusEq)
                } else {
                    self.add_token(TokenType::Minus)
                }
            }
            '*' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::StarEq)
                } else {
                    self.add_token(TokenType::Star)
                }
            }
            '/' => {
                if self.next_matches('/') {
                    self.comment()
                } else if self.next_matches('*') {
                    self.multi_line_comment()
                } else if self.next_matches('=') {
                    self.add_token(TokenType::SlashEq)
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '!' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::Ne)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::EqEq)
                } else {
                    self.add_token(TokenType::Eq)
                }
            }
            '<' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::Lte)
                } else {
                    self.add_token(TokenType::Lt)
                }
            }
            '>' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::Gte)
                } else {
                    self.add_token(TokenType::Gt)
                }
            }
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,
            '\'' => self.char(),
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error(format!("Unexpected symbol '{c}'"));
                    self.advance();
                }
            }
        }
    }

    fn add_token(&mut self, ty: TokenType) {
        let lexeme = String::from(&self.source[self.start..self.cursor]);
        self.tokens.push(Token::new(ty, lexeme, self.line))
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return 0 as char;
        }

        let next = self.chars[self.cursor];
        self.cursor += 1;
        next
    }

    fn error(&self, msg: String) {
        crate::error(self.line, msg);
        crate::set_error_found(true)
    }

    fn char(&mut self) {
        self.advance();
        self.start += 1;

        if self.peek() != '\'' {
            self.error("Unterminated character literal".to_string());
            self.advance();
            return;
        }

        self.add_token(TokenType::Char);
        self.advance();
    }

    fn string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }

        let error = || self.error("Unterminated string literal".to_string());

        if self.is_at_end() {
            error();
        } else if self.peek() == '"' {
            self.advance();

            let lexeme = String::from(&self.source[self.start + 1..self.cursor - 1]);

            self.tokens
                .push(Token::new(TokenType::String, lexeme, self.line));
        } else {
            error();
        }
    }

    fn number(&mut self) {
        while !self.is_at_end() && self.is_digit(self.peek()) {
            self.advance();
        }

        if self.is_at_end() {
            self.add_token(TokenType::Number);
            return;
        }

        if self.peek() == '.' {
            self.advance();

            while !self.is_at_end() && self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number);
    }

    fn identifier(&mut self) {
        while !self.is_at_end() && self.is_alnum(self.peek()) {
            self.advance();
        }

        let lexeme = String::from(&self.source[self.start..self.cursor]);

        if self.keywords.contains_key(&lexeme) {
            self.add_token(*self.keywords.get(&lexeme).unwrap())
        } else {
            self.add_token(TokenType::Ident)
        }
    }

    fn comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn multi_line_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '*' {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        let error = || self.error("Unterminated multi-line comment".to_string());

        if self.is_at_end() {
            error();
        } else if self.peek() == '*' {
            self.advance();

            if self.peek() == '/' {
                self.advance();
            } else {
                self.error(format!("Expected '/' found '{}'", self.peek()));
            }
        } else {
            error();
        }
    }

    fn next_matches(&mut self, c: char) -> bool {
        let res = self.peek() == c;
        if res {
            self.advance();
        }

        res
    }

    fn peek(&self) -> char {
        self.chars[self.cursor]
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
    }

    fn is_alnum(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }
}
