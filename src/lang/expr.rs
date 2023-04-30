use std::fmt::Display;

use super::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Char(char),
    Null,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, TokenType, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(TokenType, Box<Expr>),
    Literal(Literal),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(lhs, op, rhs) => f.write_fmt(format_args!("Binary({lhs}, {op:?}, {rhs})")),
            Expr::Grouping(expr) => f.write_fmt(format_args!("Grouping({expr})")),
            Expr::Unary(op, expr) => f.write_fmt(format_args!("Unary({op:?}, {expr})")),
            Expr::Literal(literal) => match literal {
                Literal::Number(x) => f.write_fmt(format_args!("Literal({x})")),
                Literal::String(x) => f.write_fmt(format_args!("Literal({x})")),
                Literal::Bool(x) => f.write_fmt(format_args!("Literal({x})")),
                Literal::Char(x) => f.write_fmt(format_args!("Literal({x})")),
                Literal::Null => f.write_fmt(format_args!("Literal(Null)")),
            },
        }
    }
}
