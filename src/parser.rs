use crate::ast::{Expr, Literal, Operator};
use crate::lexer::Token;
use std::iter::Peekable;

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
                                value: Box::new(Expr::Literal(Literal::String(s.clone()))),
                            },
                            Token::IntegerLiteral(n) => Expr::VariableDeclaration {
                                name: name.clone(),
                                value: Box::new(Expr::Literal(Literal::Integer(*n))),
                            },
                            _ => panic!("Esperado valor após '='"),
                        };

                        // Verifica se há um operador '+' logo depois do valor
                        while let Some(op_token) = iter.peek() {
                            let op = match op_token {
                                Token::Plus => Operator::Add,
                                Token::Minus => Operator::Subtract,
                                Token::Asterisk => Operator::Multiply,
                                Token::Slash => Operator::Divide,
                                Token::Percent => Operator::Modulo,
                                _ => break,
                            };

                            iter.next(); // Consumes the operator

                            if let Some(next_token) = iter.next() {
                                let right_expr = match next_token {
                                    Token::IntegerLiteral(n2) => {
                                        Expr::Literal(Literal::Integer(*n2))
                                    }
                                    Token::Identifier(name) => Expr::Identifier(name.clone()),
                                    _ => panic!(
                                        "Esperado número ou variável após operador matemático"
                                    ),
                                };
                                literal = Expr::BinaryOperation {
                                    left: Box::new(literal),
                                    op,
                                    right: Box::new(right_expr),
                                }
                            }
                        }

                        expressions.push(literal);
                    }
                }
            }
        } else if *token == Token::If {
            if let Some(condition) = iter.next() {
                let condition_expr = match condition {
                    Token::IntegerLiteral(n) => Expr::Literal(Literal::Integer(*n)),
                    Token::Identifier(name) => Expr::Identifier(name.clone()),
                    _ => panic!("Expressão inválida dentro do if"),
                };

                let tokens_vec: Vec<Token> = iter.map(|t| t.clone()).collect();
                let then_branch = parse(&tokens_vec);
                let else_branch = if let Some(Token::Else) = iter.peek() {
                    iter.next(); // Consome 'else'
                    let parsed_else = parse(&iter.by_ref().cloned().collect::<Vec<_>>());
                    parsed_else.into_iter().last()
                } else {
                    None
                };

                expressions.push(Expr::Conditional {
                    condition: Box::new(condition_expr),
                    then_branch: Box::new(then_branch.into_iter().last().unwrap()),
                    else_branch: else_branch.map(Box::new),
                });
            }
        }
    }
    expressions
}
