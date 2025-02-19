use crate::ast::{Expr, Literal, Operator};
use crate::lexer::Token;
use std::iter::Peekable;

// fn parse_if_statement(&mut self) -> Result<AstNode, String> {
//     self.consume(Token::If)?; // Consome a palavra-chave "if"
//     let condition = self.parse_expression()?; // Pega a condição do if

//     self.consume(Token::LeftBrace)?; // Consome '{'

//     let mut body = Vec::new();
//     while let Some(token) = self.tokens.peek() {
//         if *token == Token::RightBrace {
//             break;
//         }
//         body.push(self.parse_expression()?);
//     }

//     self.consume(Token::RightBrace)?; // Consome '}'

//     // Se o corpo do if estiver vazio, retorna um erro
//     if body.is_empty() {
//         return Err("Bloco 'if' vazio encontrado durante a análise".into());
//     }

//     Ok(AstNode::IfStatement {
//         condition: Box::new(condition),
//         body,
//     })
// }

pub fn parse(tokens: &[Token]) -> Vec<Expr> {
    let mut expressions = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        // Iter error
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

                // Coletamos os tokens do bloco `if` corretamente
                let mut then_tokens = Vec::new();
                let mut brace_count = 0; // Contador de blocos para garantir captura correta

                while let Some(token) = iter.peek() {
                    match token {
                        Token::LeftBrace => {
                            brace_count += 1;
                            then_tokens.push(iter.next().unwrap().clone());
                        }
                        Token::RightBrace => {
                            brace_count -= 1;
                            then_tokens.push(iter.next().unwrap().clone());
                            if brace_count == 0 {
                                break; // Sai quando fechamos todas as chaves
                            }
                        }
                        _ if brace_count > 0 => {
                            then_tokens.push(iter.next().unwrap().clone());
                        }
                        _ => break, // Se não temos `{}`, apenas sai
                    }
                }

                let then_branch = parse(&then_tokens);
                let else_branch = if let Some(Token::Else) = iter.peek() {
                    iter.next(); // Consome 'else'

                    let mut else_tokens = Vec::new();
                    let mut brace_count = 0;

                    while let Some(token) = iter.peek() {
                        match token {
                            Token::LeftBrace => {
                                brace_count += 1;
                                else_tokens.push(iter.next().unwrap().clone());
                            }
                            Token::RightBrace => {
                                brace_count -= 1;
                                else_tokens.push(iter.next().unwrap().clone());
                                if brace_count == 0 {
                                    break;
                                }
                            }
                            _ if brace_count > 0 => {
                                else_tokens.push(iter.next().unwrap().clone());
                            }
                            _ => break,
                        }
                    }

                    let parsed_else = parse(&else_tokens);
                    parsed_else.into_iter().last()
                } else {
                    None
                };

                if let Some(last_expr) = then_branch.into_iter().last() {
                    expressions.push(Expr::Conditional {
                        condition: Box::new(condition_expr),
                        then_branch: Box::new(last_expr),
                        else_branch: else_branch.map(Box::new),
                    });
                } else {
                    panic!("Bloco 'if' vazio encontrado durante a análise");
                }
            }
        }
    }
    expressions
}
