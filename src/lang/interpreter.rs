use super::{Expr, Literal, TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, expr: Expr) -> Literal {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.visit_binary(*lhs, op, *rhs),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Unary(op, expr) => self.visit_unary(op, *expr),
            Expr::Literal(literal) => literal,
        }
    }

    fn visit_binary(&self, lhs: Expr, op: TokenType, rhs: Expr) -> Literal {
        let left = self.evaluate(lhs);
        let right = self.evaluate(rhs);

        let lvalue = match left {
            Literal::Number(x) => Box::new(x),
            _ => Box::new(1.0),
        };
        let rvalue = match right {
            Literal::Number(x) => Box::new(x),
            _ => Box::new(2.0),
        };

        type TT = TokenType;
        match op {
            TT::Plus => Literal::Number(*lvalue + *rvalue),
            TT::Minus => Literal::Number(*lvalue - *rvalue),
            TT::Star => Literal::Number(*lvalue * *rvalue),
            TT::Slash => Literal::Number(*lvalue / *rvalue),
            _ => Literal::Number(1.0),
        }
    }

    fn visit_unary(&self, op: TokenType, expr: Expr) -> Literal {
        let value = self.evaluate(expr);

        let rvalue = match value {
            Literal::Number(x) => Box::new(x),
            _ => Box::new(1.0),
        };

        if let TokenType::Bang = op {
            return value;
        } else if let TokenType::Minus = op {
            return Literal::Number(-(*rvalue));
        }

        value
    }
}
