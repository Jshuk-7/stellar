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

#[derive(Debug, Clone, Copy)]
pub enum LogicalOp {
    And,
    Or,
}

impl LogicalOp {
    pub fn from(ty: TokenType) -> Self {
        match ty {
            TokenType::And => LogicalOp::And,
            TokenType::Or => LogicalOp::Or,
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
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
    Variable(String),
    Assign(String, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(lhs, op, rhs) => write!(f, "Binary({lhs}, {op:?}, {rhs})"),
            Expr::Grouping(expr) => write!(f, "Grouping({expr})"),
            Expr::Logical(lhs, op, rhs) => write!(f, "Logical({lhs}, {op:?}, {rhs})"),
            Expr::Unary(op, expr) => write!(f, "Unary({op:?}, {expr})"),
            Expr::Literal(literal) => match literal {
                Literal::Number(x) => write!(f, "Literal({x})"),
                Literal::String(x) => write!(f, "Literal({x})"),
                Literal::Bool(x) => write!(f, "Literal({x})"),
                Literal::Char(x) => write!(f, "Literal({x})"),
                Literal::Null => write!(f, "Literal(Null)"),
            },
            Expr::Variable(name) => write!(f, "Variable({name})"),
            Expr::Assign(name, expr) => write!(f, "Assign({name}, {expr})"),
        }
    }
}
