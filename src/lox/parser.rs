use super::expr::{Assignment, Binary, Expr, Grouping, Literal, Logical, Unary, Var};
use super::stmt::{self, Block, IfStmt, Stmt, While};
use super::token::{self, Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub struct Parser<'a> {
    token_list: Peekable<Iter<'a, Token>>,
    error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(borrowed_token_list: &'a [Token]) -> Self {
        Parser {
            token_list: borrowed_token_list.iter().peekable(),
            error: false,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while let Some(parsed_expression) = self.next_stmt() {
            statements.push(parsed_expression);
        }

        statements
    }

    fn next_stmt(&mut self) -> Option<Stmt> {
        let peek_token = self.token_list.peek()?;

        match peek_token.t_type {
            TokenType::Print => {
                self.token_list.next();
                self.print_statement()
            }
            TokenType::Var => {
                self.token_list.next();
                self.variable_declaration()
            }
            TokenType::LeftBrace => {
                self.token_list.next();
                self.block_statement()
            }
            TokenType::If => {
                self.token_list.next();
                self.if_statement()
            }
            TokenType::While => {
                self.token_list.next();
                self.while_statement()
            }
            _ => self.stmt_expr(),
        }
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let value = self.expression();
        let next_token = self.token_list.peek();
        if next_token == None || value == None {
            return None;
        }

        if next_token?.t_type != TokenType::Semicolon {
            println!("Expect ; after value")
        }

        Some(Stmt::Print(value.unwrap()))
    }

    fn variable_declaration(&mut self) -> Option<Stmt> {
        let name = self.token_list.next()?;

        let next_token = self.token_list.peek();
        if next_token == None {
            println!("Expect ; after value");
            return None;
        }

        let next_token = next_token.unwrap();

        if next_token.t_type == TokenType::Semicolon {
            return Some(self.empty_init(name));
        }

        if next_token.t_type != TokenType::Equal {
            println!("Expected = or ; after var");
            return None;
        }

        let next_token = self.token_list.next();

        let expr_value = self.expression();

        let value = if expr_value == None {
            let token = Token::new(
                TokenType::Nil,
                "".to_owned(),
                token::Literal::None,
                next_token?.line,
            );
            let literal = Literal { token };

            Expr::Literal(literal)
        } else {
            expr_value.unwrap()
        };

        let variable = stmt::Var {
            value,
            name: name.lexeme.to_owned(),
        };

        Some(Stmt::Declaration(variable))
    }

    fn empty_init(&mut self, name: &Token) -> Stmt {
        let token = Token::new(
            TokenType::Nil,
            "".to_owned(),
            token::Literal::None,
            name.line,
        );
        let literal = Literal { token };
        let value = Expr::Literal(literal);

        let variable = stmt::Var {
            value,
            name: name.lexeme.to_owned(),
        };

        Stmt::Declaration(variable)
    }

    fn block_statement(&mut self) -> Option<Stmt> {
        let mut statements = Vec::new();

        while let Some(next_token) = self.token_list.peek() {
            if next_token.t_type == TokenType::RightBrace {
                break;
            }

            let next_stmt = self.next_stmt()?;
            statements.push(next_stmt);
        }

        if self.token_list.peek() == None {
            panic!("Missing closing bracket");
        }

        Some(Stmt::Block(Block {
            stmt_vec: statements,
        }))
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::LeftParen {
            println!("Expect ( after if")
        }
        self.token_list.next();

        let condition = self.expression()?;
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::RightParen {
            println!("Expect ) after if condition")
        }
        self.token_list.next();

        let if_stmt = self.next_stmt()?;
        let mut else_stmt = None;
        let next_token = self.token_list.peek();
        if next_token?.t_type == TokenType::Else {
            self.token_list.next();
            else_stmt = self.next_stmt();
        }
        Some(Stmt::If(IfStmt {
            condition,
            truth_branch: Box::new(if_stmt),
            false_branch: Box::new(else_stmt),
        }))
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::LeftParen {
            println!("Expect ( after while")
        }
        self.token_list.next();

        let condition = self.expression()?;
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::RightParen {
            println!("Expect ) after while condition")
        }
        self.token_list.next();

        if self.token_list.peek()?.t_type != TokenType::LeftBrace {
            println!("Expected a block after while condition");
            return None;
        }

        let next_stmt = self.next_stmt()?;

        if self.token_list.next()?.t_type != TokenType::RightBrace {
            println!("Expected a block after while condition");
            return None;
        }
        let body = Box::new(next_stmt);
        let result = Stmt::While(While { condition, body });
        Some(result)
    }

    fn stmt_expr(&mut self) -> Option<Stmt> {
        let expr = self.assignment();
        let next_token = self.token_list.peek();
        if next_token?.t_type != TokenType::Semicolon {
            println!("Expect ; after expression")
        }
        self.token_list.next();
        if expr == None {
            None
        } else {
            Some(Stmt::Expr(expr.unwrap()))
        }
    }

    fn expression(&mut self) -> Option<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Expr> {
        let possible_expr = self.logic_or();

        if self.token_list.peek().is_none() {
            return possible_expr;
        }

        let next_token = self.token_list.peek().unwrap();

        if let TokenType::Equal = next_token.t_type {
            self.token_list.next();

            if possible_expr == None {
                panic!("Invalid assignment");
            }

            let value = self.assignment();
            if value == None {
                panic!("Invalid value on the right hand side");
            }

            let value = Box::new(value.unwrap());
            let expr = possible_expr.unwrap();

            if let Expr::Var(var) = expr {
                let name = var.name;
                let assignment = Expr::Assignment(Assignment { name, value });
                return Some(assignment);
            }

            panic!("Invalid assignment")
        } else {
            possible_expr
        }
    }

    fn logic_or(&mut self) -> Option<Expr> {
        let mut expr = self.logic_and();

        while let TokenType::Or = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let left = Box::new(expr?);
            let right = Box::new(self.logic_and()?);

            expr = Some(Expr::Logical(Logical {
                left,
                right,
                operator,
            }));
        }

        expr
    }
    fn logic_and(&mut self) -> Option<Expr> {
        let mut expr = self.equality();

        while let TokenType::And = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let left = Box::new(expr?);
            let right = Box::new(self.equality()?);

            expr = Some(Expr::Logical(Logical {
                left,
                right,
                operator,
            }));
        }

        expr
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison();

        if self.token_list.peek() == None {
            return expr;
        }

        if let TokenType::BangEqual | TokenType::EqualEqual = self.token_list.peek()?.t_type {
            let operator = self.token_list.next()?.clone();
            let right = Box::new(self.comparison()?);
            let left = Box::new(expr?);
            expr = Some(Expr::Binary(Binary {
                left,
                right,
                operator,
            }));
        }

        expr
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.addition();
        if self.token_list.peek() == None {
            return expr;
        }

        if let TokenType::Greater
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

        if self.token_list.peek() == None {
            return Some(expr);
        }
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
            TokenType::Identifier => {
                let name = self.token_list.next()?.clone();
                Some(Expr::Var(Var { name }))
            }
            TokenType::Number
            | TokenType::String
            | TokenType::False
            | TokenType::True
            | TokenType::Nil => {
                let token = self.token_list.next()?.clone();
                Some(Expr::Literal(Literal { token }))
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
            TokenType::EOF | TokenType::Semicolon => None,
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
    use super::*;
    use crate::lox::expr::Literal as ExprLiteral;
    use crate::lox::token::Literal;

    #[test]
    fn literal_string() {
        let string_token = Token::new(
            TokenType::String,
            "string".to_owned(),
            Literal::String("string".into()),
            1,
        );

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

        let expected_expr = Expr::Literal(ExprLiteral {
            token: string_token.clone(),
        });

        let tokens = vec![string_token, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn negation() {
        let number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::Bang, "!".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: number.clone(),
        });
        let expected_expr = Expr::Unary(Unary {
            expr: Box::new(left),
            operator: operator.clone(),
        });

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let tokens = vec![operator, number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn addition() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let operator = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);

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

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn equality() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::EqualEqual, "==".to_owned(), Literal::None, 1);

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

        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);
        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn comparison() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let second_number = Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1);
        let operator = Token::new(TokenType::Greater, ">".to_owned(), Literal::None, 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

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

        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn multiplication() {
        let first_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let operator = Token::new(TokenType::Star, "*".to_owned(), Literal::None, 1);
        let second_number = Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1);
        let semicolon = Token::new(TokenType::Semicolon, ";".to_owned(), Literal::None, 1);

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

        let tokens = vec![first_number, operator, second_number, semicolon];

        let mut parser = Parser::new(&tokens);
        let mut stmt = parser.parse();
        if let Some(Stmt::Expr(expr)) = stmt.pop() {
            assert_eq!(expr, expected_expr);
        } else {
            unreachable!()
        }
    }
}
