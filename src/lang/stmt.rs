use super::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Expr>),
}
