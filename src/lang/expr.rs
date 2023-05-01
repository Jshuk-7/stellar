use std::fmt::Display;

use super::TokenType;

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Equal,
    NotEqual,
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl BinaryOp {
    pub fn from(ty: TokenType) -> Self {
        match ty {
            TokenType::EqEq => BinaryOp::Equal,
            TokenType::Ne => BinaryOp::NotEqual,
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Sub,
            TokenType::Star => BinaryOp::Mul,
            TokenType::Slash => BinaryOp::Div,
            TokenType::Gt => BinaryOp::Gt,
            TokenType::Gte => BinaryOp::Gte,
            TokenType::Lt => BinaryOp::Lt,
            TokenType::Lte => BinaryOp::Lte,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Bang,
    Minus,
}

impl UnaryOp {
    pub fn from(ty: TokenType) -> Self {
        match ty {
            TokenType::Bang => UnaryOp::Bang,
            TokenType::Minus => UnaryOp::Minus,
            _ => unreachable!(),
        }
    }
}

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
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
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
