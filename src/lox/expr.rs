use super::token::Token;

pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
}

pub struct Grouping {
    pub expr: Box<Expr>,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub token: Token,
}
pub struct Unary {
    pub expr: Box<Expr>,
    pub token: Token,
}

pub struct Literal {
    pub token: Token,
}
