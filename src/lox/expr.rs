use super::token::Token;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Literal(Literal),
    Logical(Logical),
    Var(Var),
    Assignment(Assignment),
    Unary(Unary),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Logical {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Unary {
    pub expr: Box<Expr>,
    pub operator: Token,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    pub token: Token,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Var {
    pub name: Token,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Assignment {
    pub name: Token,
    pub value: Box<Expr>,
}
