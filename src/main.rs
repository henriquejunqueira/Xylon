// Program entry point

mod ast;
mod lexer;
mod parser;
mod translator;

use lexer::tokenize;
use parser::parse;

fn main() {
    // let source_code = "var firstName = 'Henrique';\nvar age = 30;";
    // let source_code = "Konstante alter = 25;";
    let source_code = "variável nome = 'Henrique';\nvariável idade = 30;";
    let tokens = tokenize(source_code);
    let ast = parse(&tokens);

    println!("Tokens: {:?}", tokens);
    println!("AST: {:?}", ast);
}
