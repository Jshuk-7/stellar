use super::{Expr, Literal, Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expr()
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

    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(vec![TokenType::Ne, TokenType::EqEq]) {
            let operator = self.previous();
            let rhs = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator.ty, Box::new(rhs));
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
            let operator = self.previous();
            let rhs = self.term();
            expr = Expr::Binary(Box::new(expr), operator.ty, Box::new(rhs));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let rhs = self.unary();
            expr = Expr::Binary(Box::new(expr), operator.ty, Box::new(rhs));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let rhs = self.factor();
            expr = Expr::Binary(Box::new(expr), operator.ty, Box::new(rhs));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let rhs = self.unary();
            return Expr::Unary(operator.ty, Box::new(rhs));
        }

        self.atom().unwrap()
    }

    fn atom(&mut self) -> Option<Expr> {
        if self.matches(vec![TokenType::True]) {
            return Some(Expr::Literal(Literal::Bool(true)));
        } else if self.matches(vec![TokenType::False]) {
            return Some(Expr::Literal(Literal::Bool(false)));
        } else if self.matches(vec![TokenType::Null]) {
            return Some(Expr::Literal(Literal::Null));
        } else if self.matches(vec![TokenType::Number]) {
            let literal_value = self.previous().lexeme.parse::<f64>().unwrap();
            return Some(Expr::Literal(Literal::Number(literal_value)));
        } else if self.matches(vec![TokenType::String]) {
            return Some(Expr::Literal(Literal::String(self.previous().lexeme)));
        } else if self.matches(vec![TokenType::LParen]) {
            let expr = self.expr();
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

        self.error(self.peek(), msg);
        None
    }

    fn error(&self, token: Token, msg: String) {
        crate::error(token.line, format!("at '{}', {msg}", token.lexeme));
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
