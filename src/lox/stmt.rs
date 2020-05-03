use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub enum Stmt {
    Declaration(Var),
    Expr(Expr),
    Print(Expr),
    Block(Block),
    If(IfStmt),
}

#[derive(PartialEq, Debug)]
pub struct Block {
    pub stmt_vec: Vec<Stmt>,
}

#[derive(PartialEq, Debug)]
pub struct Var {
    pub value: Expr,
    pub name: String,
}

#[derive(PartialEq, Debug)]
pub struct IfStmt {
    pub truth_branch: Box<Stmt>,
    pub false_branch: Box<Option<Stmt>>,
    pub condition: Expr,
}
