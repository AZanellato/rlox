use super::expr::Expr;

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Declaration(Var),
    Expr(Expr),
    Print(Expr),
    Block(Block),
    If(IfStmt),
    While(While),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub stmt_vec: Vec<Stmt>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Var {
    pub value: Expr,
    pub name: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct IfStmt {
    pub truth_branch: Box<Stmt>,
    pub false_branch: Box<Option<Stmt>>,
    pub condition: Expr,
}
