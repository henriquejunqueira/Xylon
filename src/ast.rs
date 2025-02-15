// Definition of the AST structure

use std::fmt;

#[allow(dead_code)]
pub enum Expr {
    VariableDeclaration {
        name: String,
        value: Literal,
    },
    BinaryOperation {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
}

#[derive(Debug)]
pub enum Operator {
    Add,      // addition (+)
    Subtract, // subtraction (-)
    Multiply, // multiplication (*)
    Divide,   // division (/)
    Modulo,   // module (%)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::VariableDeclaration { name, value } => {
                write!(f, "Variable '{}': {}", name, value)
            }
            Expr::BinaryOperation { left, op, right } => {
                write!(f, "{} {} {}", left, op, right)
            }
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Modulo => "%",
        };
        write!(f, "{}", symbol)
    }
}
