use super::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Let(String, Option<Box<Expr>>),
    Block(Vec<Stmt>),
}
