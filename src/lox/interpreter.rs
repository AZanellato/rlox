use super::expr::{Binary, Expr, Literal, Unary};
use super::stmt::Stmt;
use super::token;
use derive_more::Display;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(PartialEq, PartialOrd, Debug, Display, Clone)]
pub enum Value {
    String(String),
    F64(f64),
    Boolean(bool),
    Nil,
}

struct Environment {
    env_values: HashMap<String, Value>,
}

impl Environment {
    fn new() -> Self {
        Self {
            env_values: HashMap::new(),
        }
    }

    fn define(&mut self, name: String, value: Value) {
        self.env_values.insert(name, value);
    }

    fn get(&mut self, name: String) -> Option<&Value> {
        self.env_values.get(&name)
    }
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Environment::new();
        Self { env }
    }
}

pub fn evaluate_node(stmt: Stmt) -> Value {
    match stmt {
        Stmt::Expr(expr) => evaluate_expression(expr),
        Stmt::Print(expr) => evaluate_print(expr),
        Stmt::Declaration(expr) => {
            println!("{:?}", expr);
            println!("Not finished");
            Value::Nil
        }
    }
}

fn evaluate_print(expr: Expr) -> Value {
    let value = evaluate_expression(expr);
    println!("{}", value);
    Value::Nil
}

fn evaluate_expression(expr: Expr) -> Value {
    match expr {
        Expr::Literal(expr) => evalute_literal(expr),
        Expr::Unary(expr) => evaluate_unary(expr),
        Expr::Binary(expr) => evaluate_binary(expr),
        _ => panic!("Not implemented yet"),
    }
}

fn evalute_literal(expr: Literal) -> Value {
    match expr.token.literal {
        token::Literal::String(string) => Value::String(string),
        token::Literal::F64(f64) => Value::F64(f64),
        token::Literal::Boolean(boolean) => Value::Boolean(boolean),
        _ => Value::Nil,
    }
}

fn evaluate_unary(unary_expr: Unary) -> Value {
    let value = evaluate_expression(*unary_expr.expr);

    let new_value = match unary_expr.operator.t_type {
        token::TokenType::Minus => -value,
        token::TokenType::Bang => !value,
        _ => Value::Nil,
    };
    println!("{:?}", new_value);
    new_value
}

fn evaluate_binary(expr: Binary) -> Value {
    let left_value = evaluate_expression(*expr.left);
    let right_value = evaluate_expression(*expr.right);

    match expr.operator.t_type {
        token::TokenType::Plus => left_value + right_value,
        token::TokenType::Minus => left_value - right_value,
        token::TokenType::Slash => left_value / right_value,
        token::TokenType::Star => left_value * right_value,
        token::TokenType::Greater => Value::Boolean(left_value > right_value),
        token::TokenType::GreaterEqual => Value::Boolean(left_value >= right_value),
        token::TokenType::Less => Value::Boolean(left_value < right_value),
        token::TokenType::LessEqual => Value::Boolean(left_value <= right_value),
        token::TokenType::EqualEqual => Value::Boolean(left_value == right_value),
        token::TokenType::BangEqual => Value::Boolean(left_value != right_value),
        _ => panic!("Not implemented"),
    }
}

fn truthyness(value: Value) -> bool {
    match value {
        Value::Boolean(boolean) => boolean,
        Value::Nil => false,
        _ => true,
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left + right),
            (Value::String(left), Value::String(right)) => {
                let mut new_string = left;
                new_string.push_str(&right);
                Value::String(new_string)
            }
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left - right),
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left / right),
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::F64(left), Value::F64(right)) => Value::F64(left * right),
            (_, _) => panic!("Not implemented"),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::F64(value) => Value::F64(-value),
            _ => panic!("Expected a number, got: {:?}", self),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        Value::Boolean(!truthyness(self))
    }
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::lox::expr::Literal as ExprLiteral;
    use crate::lox::token::{Literal, Token, TokenType};

    #[test]
    fn literal_string() {
        let expr = Expr::Literal(ExprLiteral {
            token: Token::new(
                TokenType::String,
                "string".to_owned(),
                Literal::String("string".into()),
                1,
            ),
        });

        let value = self::evaluate_expression(expr);
        assert_eq!(value, Value::String("string".into()));
    }

    #[test]
    fn negation() {
        let operator = Token::new(TokenType::Bang, "!".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let expr = Expr::Unary(Unary {
            expr: Box::new(left),
            operator,
        });

        let value = self::evaluate_expression(expr);
        assert_eq!(value, Value::Boolean(false));
    }

    #[test]
    fn addition() {
        let operator = Token::new(TokenType::Plus, "+".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = self::evaluate_expression(expr);
        assert_eq!(value, Value::F64(3.0));
    }

    #[test]
    fn equality() {
        let operator = Token::new(TokenType::EqualEqual, "==".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = self::evaluate_expression(expr);
        assert_eq!(value, Value::Boolean(true))
    }

    #[test]
    fn comparison() {
        let operator = Token::new(TokenType::Greater, ">".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "1".to_owned(), Literal::F64(1.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "2".to_owned(), Literal::F64(2.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = self::evaluate_expression(expr);
        assert_eq!(value, Value::Boolean(false))
    }

    #[test]
    fn multiplication() {
        let operator = Token::new(TokenType::Star, "*".to_owned(), Literal::None, 1);

        let left = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "3".to_owned(), Literal::F64(3.0), 1),
        });
        let right = Expr::Literal(super::Literal {
            token: Token::new(TokenType::Number, "3".to_owned(), Literal::F64(3.0), 1),
        });
        let expr = Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        });

        let value = self::evaluate_expression(expr);
        assert_eq!(value, Value::F64(9.0));
    }
}
