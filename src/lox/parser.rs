use super::expr::{Binary, Expr, Grouping, Literal, Unary};
use super::token::{Token, TokenList, TokenType};
use std::iter::{Iterator, Peekable};

struct Parser<'a> {
    token_list: Peekable<TokenList>,
    error: bool,
}

impl<'a> Parser<'a> {
    fn new(token_list: Vec<Token>) -> Self {
        Parser {
            token_list,
            error: false,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        let peek = self.token_list.peek();

        while let TokenType::BangEqual | TokenType::EqualEqual = peek {
            let operator = self.token_list.next();
            let right = Box::new(self.comparison());
            let left = Box::new(expr);
            expr = Expr::Binary(Binary {
                left,
                right,
                operator,
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();

        let peek = self.token_list.peek();

        while let TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual = peek
        {
            let right = Box::new(self.multiplication());
            let operator = self.token_list.next();
            let left = Box::new(expr);
            expr = Expr::Binary(Binary {
                left,
                right,
                operator,
            });
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        let peek = self.token_list.peek();

        while let TokenType::Minus | TokenType::Plus = peek {
            let operator = self.token_list.next();
            let right = Box::new(self.multiplication());
            let left = Box::new(expr);
            expr = Expr::Binary(Binary {
                left,
                right,
                operator,
            });
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        let peek = self.token_list.peek();

        while let TokenType::Slash | TokenType::Star = peek {
            let operator = self.token_list.next();
            let right = Box::new(self.unary());
            let left = Box::new(expr);
            expr = Expr::Binary(Binary {
                left,
                right,
                operator,
            });
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let peek = self.token_list.peek();

        if let TokenType::Bang | TokenType::Minus = peek {
            let operator = self.token_list.next();
            let expr = Box::new(self.unary());
            Expr::Unary(Unary { expr, operator })
        } else {
            self.primary().unwrap()
        }
    }

    fn primary(&mut self) -> Option<Expr> {
        let peek = self.token_list.peek();

        match peek {
            TokenType::Number
            | TokenType::String
            | TokenType::False
            | TokenType::True
            | TokenType::Nil => Some(Expr::Literal(Literal {
                token: self.token_list.next(),
            })),
            TokenType::LeftParen => {
                let expr = self.expression();
                if let TokenType::RightParen = self.token_list.next() {
                    return Some(Expr::Grouping(Grouping {
                        expr: Box::new(expr),
                    }));
                }
                self.error = true;
                println!("Expecting ')' after '(' and expression");
                None
            }
            _ => {
                self.error = true;
                println!("Expecting an expression");
                None
            }
        }
    }
}
