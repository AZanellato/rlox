use super::token::Token;

#[derive(Debug)]
pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Debug)]
pub struct Unary {
    pub expr: Box<Expr>,
    pub operator: Token,
}

#[derive(Debug)]
pub struct Literal {
    pub token: Token,
}
