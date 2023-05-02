use super::{BinaryOp, Expr, Literal, Stmt, Token, TokenType, UnaryOp};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration());
        }

        statements
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.index += 1;
        }

        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn declaration(&mut self) -> Stmt {
        if self.matches(vec![TokenType::Let]) {
            return self.variable_declaration();
        }

        self.statement()
    }

    fn statement(&mut self) -> Stmt {
        if self.matches(vec![TokenType::If]) {
            return self.if_statement();
        } else if self.matches(vec![TokenType::Print]) {
            return self.print_statement();
        } else if self.matches(vec![TokenType::LCurly]) {
            return Stmt::Block(self.block());
        }

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression".to_string(),
        );

        Stmt::Expr(Box::new(expr))
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(
            TokenType::LParen,
            "Expected '(' before expression".to_string(),
        );

        let condition = self.expression();

        self.consume(
            TokenType::RParen,
            "Expected ')' after expression".to_string(),
        );

        self.consume(
            TokenType::LCurly,
            "Expected '{' after condition".to_string(),
        );

        let main_branch = self.statement();

        self.consume(
            TokenType::RCurly,
            "Expected '}' after statement".to_string(),
        );

        let mut else_branch = None;
        if self.matches(vec![TokenType::Else]) {
            self.consume(
                TokenType::LCurly,
                "Expected '{' after else branch".to_string(),
            );

            else_branch = Some(Box::new(self.statement()));

            self.consume(
                TokenType::RCurly,
                "Expected '}' after statement".to_string(),
            );
        }

        Stmt::If(Box::new(condition), Box::new(main_branch), else_branch)
    }

    fn variable_declaration(&mut self) -> Stmt {
        let name = self
            .consume(TokenType::Ident, "Expected identifier".to_string())
            .unwrap();

        let mut initializer = None;
        if self.matches(vec![TokenType::Eq]) {
            initializer = Some(Box::new(self.expression()));
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after declaration".to_string(),
        );

        Stmt::Let(name.lexeme, initializer)
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RCurly) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RCurly, "Expected '}' after block".to_string());

        statements
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression".to_string(),
        );

        Stmt::Print(Box::new(expr))
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();

        if self.matches(vec![TokenType::Eq]) {
            let equals_op = self.previous();
            let value = self.assignment();

            if let Expr::Variable(name) = expr {
                return Expr::Assign(name, Box::new(value));
            }

            self.error(equals_op, format!("lvalue required"));
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(vec![TokenType::Ne, TokenType::EqEq]) {
            let operator = BinaryOp::from(self.previous().ty);
            let rhs = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(vec![
            TokenType::Gt,
            TokenType::Gte,
            TokenType::Lt,
            TokenType::Lte,
        ]) {
            let operator = BinaryOp::from(self.previous().ty);
            let rhs = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![TokenType::Star, TokenType::Slash]) {
            let operator = BinaryOp::from(self.previous().ty);
            let rhs = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = BinaryOp::from(self.previous().ty);
            let rhs = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(rhs));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = UnaryOp::from(self.previous().ty);
            let rhs = self.unary();
            return Expr::Unary(operator, Box::new(rhs));
        }

        self.atom().unwrap()
    }

    fn atom(&mut self) -> Option<Expr> {
        if self.matches(vec![TokenType::Null]) {
            return Some(Expr::Literal(Literal::Null));
        } else if self.matches(vec![TokenType::Number]) {
            let literal_value = self.previous().lexeme.parse::<f64>().unwrap();
            return Some(Expr::Literal(Literal::Number(literal_value)));
        } else if self.matches(vec![TokenType::String]) {
            return Some(Expr::Literal(Literal::String(self.previous().lexeme)));
        } else if self.matches(vec![TokenType::True]) {
            return Some(Expr::Literal(Literal::Bool(true)));
        } else if self.matches(vec![TokenType::False]) {
            return Some(Expr::Literal(Literal::Bool(false)));
        } else if self.matches(vec![TokenType::Char]) {
            let character = self.previous().lexeme.parse::<char>().unwrap();
            return Some(Expr::Literal(Literal::Char(character)));
        } else if self.matches(vec![TokenType::Ident]) {
            return Some(Expr::Variable(self.previous().lexeme));
        } else if self.matches(vec![TokenType::LParen]) {
            let expr = self.expression();
            self.consume(
                TokenType::RParen,
                "Expected ')' after expression".to_string(),
            );
            return Some(Expr::Grouping(Box::new(expr)));
        }

        self.error(self.peek(), "Expected expression".to_string());
        None
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ty == TokenType::Semicolon {
                return;
            }

            type TT = TokenType;
            match self.peek().ty {
                TT::Struct | TT::Fun | TT::Let | TT::For | TT::If | TT::While | TT::Return => {
                    return
                }
                _ => (),
            }

            self.advance();
        }
    }

    fn consume(&mut self, ty: TokenType, msg: String) -> Option<Token> {
        if self.check(ty) {
            return Some(self.advance());
        }

        self.error(
            self.peek(),
            format!("{msg}, found '{}'", self.peek().lexeme),
        );

        None
    }

    fn error(&mut self, token: Token, msg: String) {
        crate::error(token.line, format!("at '{}', {msg}", token.lexeme));
        self.synchronize();
    }

    fn matches(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types.iter() {
            if self.check(*ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ty == ty
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }
}
