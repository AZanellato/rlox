use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}
