use crate::ast::{Expr, Literal, Operator};
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Vec<Expr> {
    let mut expressions = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        if *token == Token::Var {
            if let Some(Token::Identifier(name)) = iter.next() {
                if let Some(Token::Equals) = iter.next() {
                    if let Some(value) = iter.next() {
                        let mut literal = match value {
                            Token::StringLiteral(s) => Expr::VariableDeclaration {
                                name: name.clone(),
                                value: Literal::String(s.clone()),
                            },
                            Token::IntegerLiteral(n) => Expr::VariableDeclaration {
                                name: name.clone(),
                                value: Literal::Integer(*n),
                            },
                            _ => panic!("Esperado valor após '='"),
                        };

                        // Verifica se há um operador '+' logo depois do valor
                        if let Some(Token::Plus) = iter.peek() {
                            iter.next(); // Consome '+'
                            if let Some(Token::IntegerLiteral(n2)) = iter.next() {
                                literal = Expr::BinaryOperation {
                                    left: Box::new(literal),
                                    op: Operator::Add,
                                    right: Box::new(Expr::VariableDeclaration {
                                        name: "_".to_string(),
                                        value: Literal::Integer(*n2),
                                    }),
                                };
                            }
                        }

                        expressions.push(literal);
                    }
                }
            }
        }
    }
    expressions
}
