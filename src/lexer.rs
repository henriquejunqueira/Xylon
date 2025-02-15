// Tokenizer (lexer)

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Var,
    Const,
    Println, // checking
    Identifier(String),
    Equals,
    StringLiteral(String),
    IntegerLiteral(i64),
    Plus,     // new operator '+'
    Minus,    // new operator '-'
    Asterisk, // new operator '*'
    Slash,    // new operator '/'
    Percent,  // new operator '%'
    Semicolon,
    If,   // checking
    Else, // checking
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
                "println!" => tokens.push(Token::Println), // checking
                "if" => tokens.push(Token::If),            // checking
                "else" => tokens.push(Token::Else),        // checking
                _ => tokens.push(Token::Identifier(translated_ident)),
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
                    chars.next();
                    tokens.push(Token::StringLiteral(string));
                    break;
                }
                string.push(ch);
                chars.next();
            }
            // chars.next();
            // tokens.push(Token::StringLiteral(string));
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
        } else if c == '-' {
            tokens.push(Token::Minus);
            chars.next();
        } else if c == '*' {
            tokens.push(Token::Asterisk);
            chars.next();
        } else if c == '/' {
            chars.next();
            if let Some(&next) = chars.peek() {
                if next == '/' {
                    // Line comment (//)
                    while let Some(&ch) = chars.peek() {
                        if ch == '\n' {
                            break;
                        }

                        chars.next();
                    }
                } else if next == '*' {
                    // Block comment (/* ... */)
                    chars.next();
                    while let Some(&ch) = chars.peek() {
                        chars.next();
                        if ch == '*' {
                            if let Some(&end) = chars.peek() {
                                if end == '/' {
                                    chars.next();
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    tokens.push(Token::Slash);
                }
            } else {
                tokens.push(Token::Slash);
            }
        } else if c == '%' {
            tokens.push(Token::Percent);
            chars.next();
        } else {
            chars.next();
        }
    }
    tokens
}
