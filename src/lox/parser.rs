use super::expr::{Binary, Expr, Grouping, Literal, Unary};
use super::token::{Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    token_list: Peekable<Iter<'a, Token>>,
    error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(borrowed_token_list: &'a Vec<Token>) -> Self {
        Parser {
            token_list: borrowed_token_list.into_iter().peekable(),
            error: false,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison();

        if self.token_list.peek() == None {
            return expr;
        }

        while let TokenType::BangEqual | TokenType::EqualEqual = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.comparison()?);
            let left = Box::new(expr?);
            expr = Some(Expr::Binary(Binary {
                left,
                right,
                operator,
            }));
            if self.token_list.peek() == None {
                break;
            }
        }

        expr
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.addition();

        if self.token_list.peek() == None {
            return expr;
        }

        while let TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual = self.token_list.peek()?.t_type
        {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.multiplication()?);
            let left = Box::new(expr?);
            expr = Some(Expr::Binary(Binary {
                left,
                right,
                operator,
            }));
            if self.token_list.peek() == None {
                break;
            }
        }

        expr
    }

    fn addition(&mut self) -> Option<Expr> {
        let mut expr = self.multiplication();

        if self.token_list.peek() == None {
            return expr;
        }
        while let TokenType::Minus | TokenType::Plus = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.multiplication()?);
            let left = Box::new(expr?);
            expr = Some(Expr::Binary(Binary {
                left,
                right,
                operator,
            }));
            if self.token_list.peek() == None {
                break;
            }
        }

        expr
    }

    fn multiplication(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;

        while let TokenType::Slash | TokenType::Star = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.unary()?);
            let left = Box::new(expr);
            expr = Expr::Binary(Binary {
                left,
                right,
                operator,
            });
            if self.token_list.peek() == None {
                break;
            }
        }

        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        let peek = self.token_list.peek()?;

        if let TokenType::Bang | TokenType::Minus = peek.t_type {
            let operator = self.token_list.next()?.clone();
            let expr = Box::new(self.unary()?);
            Some(Expr::Unary(Unary { expr, operator }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Option<Expr> {
        let peek = self.token_list.peek()?;

        match peek.t_type {
            TokenType::Number
            | TokenType::String
            | TokenType::False
            | TokenType::True
            | TokenType::Nil => {
                let next = self.token_list.next()?.clone();
                Some(Expr::Literal(Literal { token: next }))
            }
            TokenType::LeftParen => {
                self.token_list.next();
                let expr = self.expression()?;
                if let TokenType::RightParen = self.token_list.next().unwrap().t_type {
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
                println!("{:?}", peek.t_type);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::lox::token::Literal;

    #[test]
    fn test_expression() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::Star, "*".to_owned(), Literal::None, 1);
        let second_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);

        let left = Expr::Literal(super::Literal {
            token: first_number.clone(),
        });
        let right = Expr::Literal(super::Literal {
            token: second_number.clone(),
        });
        let expected_expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator: operator.clone(),
        });

        let tokens = vec![first_number, operator, second_number];

        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, expected_expr);
    }
}
