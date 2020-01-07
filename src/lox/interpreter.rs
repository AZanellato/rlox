use super::expr::{Binary, Expr, Literal, Unary};
use super::token;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    String(String),
    F64(f64),
    Boolean(bool),
    Nil,
}

pub fn evaluate_node(expr: Expr) -> Value {
    match expr {
        Expr::Literal(expr) => evalute_literal(expr),
        Expr::Unary(expr) => evaluate_unary(expr),
        Expr::Binary(expr) => evaluate_binary(expr),
        _ => panic!("Not implemented yet"),
    }
}

fn evalute_literal(expr: Literal) -> Value {
    let value = match expr.token.literal {
        token::Literal::String(string) => Value::String(string),
        token::Literal::F64(f64) => Value::F64(f64),
        token::Literal::Boolean(boolean) => Value::Boolean(boolean),
        _ => Value::Nil,
    };
    println!("{:?}", value);
    value
}

fn evaluate_unary(unary_expr: Unary) -> Value {
    let value = evaluate_node(*unary_expr.expr);

    let new_value = match unary_expr.operator.t_type {
        token::TokenType::Minus => -value,
        token::TokenType::Bang => !value,
        _ => Value::Nil,
    };
    println!("{:?}", new_value);
    new_value
}

fn evaluate_binary(expr: Binary) -> Value {
    let left_value = evaluate_node(*expr.left);
    let right_value = evaluate_node(*expr.right);

    let value = match expr.operator.t_type {
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
        _ => panic!("Not implemented yet"),
    };
    println!("{:?}", value);
    value
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
