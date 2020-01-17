use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Stmt {
    Declaration(Var),
    Expr(Expr),
    Print(Expr),
}

#[derive(PartialEq, Debug)]
pub struct Var {
    pub value: Expr,
    pub name: String,
}
