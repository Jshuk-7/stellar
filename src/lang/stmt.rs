use super::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expr(Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    Let(String, Option<Box<Expr>>),
    Block(Vec<Stmt>),
    Print(Box<Expr>),
}
