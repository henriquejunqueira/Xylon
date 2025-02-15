// Definition of the AST structure

use std::fmt;

#[allow(dead_code)]
pub enum Expr {
    Identifier(String),
    VariableDeclaration {
        name: String,
        value: Box<Expr>,
    },
    BinaryOperation {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Conditional {
        condition: Box<Expr>,           // The expression inside the if
        then_branch: Box<Expr>,         // What happens if it is true
        else_branch: Option<Box<Expr>>, // What happens if false (optional)
    },
    Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
}

#[derive(Debug)]
pub enum Operator {
    Add,          // addition (+)
    Subtract,     // subtraction (-)
    Multiply,     // multiplication (*)
    Divide,       // division (/)
    Modulo,       // modulo (%)
    GreaterThan,  // greater than (>)
    LessThan,     // less than (<)
    GreaterEqual, // greater than or equal (>=)
    LessEqual,    // less than or equal (<=)
    Equal,        // equal (==)
    NotEqual,     // not equal (!=)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::VariableDeclaration { name, value } => {
                write!(f, "Variable '{}': {}", name, value)
            }
            Expr::BinaryOperation { left, op, right } => {
                write!(f, "{} {} {}", left, op, right)
            }
            Expr::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                if let Some(else_b) = else_branch {
                    write!(
                        f,
                        "if({}) {{ {} }} else {{ {} }}",
                        condition, then_branch, else_b
                    )
                } else {
                    write!(f, "if ({}) {{ {} }}", condition, then_branch)
                }
            }
            Expr::Literal(literal) => write!(f, "{:?}", literal),
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
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterEqual => ">=",
            Operator::LessEqual => "<=",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
        };
        write!(f, "{}", symbol)
    }
}
