// Tokenizer (lexer)

#[derive(Debug, PartialEq)]
pub enum Token {
    Var,
    Const,
    Identifier(String),
    Equals,
    StringLiteral(String),
    IntegerLiteral(i64),
    Plus, // New operator '+'
    Semicolon,
}

use crate::translator::Translator;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let translator = Translator::new();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_alphabetic() {
            let mut ident = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() {
                    ident.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let translated_ident = translator.translate(&ident);
            match translated_ident.as_str() {
                "var" => tokens.push(Token::Var),
                "const" => tokens.push(Token::Const),
                _ => tokens.push(Token::Identifier(ident)),
            }
            // if ident == "var" {
            //     tokens.push(Token::Var);
            // } else {
            //     tokens.push(Token::Identifier(ident));
            // }
        } else if c == '=' {
            tokens.push(Token::Equals);
            chars.next();
        } else if c == ';' {
            tokens.push(Token::Semicolon);
            chars.next();
        } else if c == '"' || c == '\'' {
            let quote = c;
            chars.next();
            let mut string = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == quote {
                    break;
                }
                string.push(ch);
                chars.next();
            }
            chars.next();
            tokens.push(Token::StringLiteral(string));
        } else if c.is_digit(10) {
            let mut number = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_digit(10) {
                    number.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::IntegerLiteral(number.parse().unwrap()));
        } else if c == '+' {
            tokens.push(Token::Plus);
            chars.next();
        } else {
            chars.next();
        }
    }
    tokens
}
