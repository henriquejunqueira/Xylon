// Program entry point

use std::fs;
use std::io;
mod ast;
mod lexer;
mod parser;
mod translator;

use lexer::tokenize;
use parser::parse;

fn main() -> io::Result<()> {
    // let source_code = "var firstName = 'Henrique';\nvar age = 30;";
    // let source_code = "Konstante alter = 25;";
    // let source_code = "variável nome = 'Henrique';\nvariável idade = 30;";
    // let tokens = tokenize(source_code);
    // let ast = parse(&tokens);

    // println!("Tokens: {:?}", tokens);
    // println!("AST: {:?}", ast);

    // Reads the contents of a "code.js" file
    //let source_code = fs::read_to_string("examples/examples.xylon")?;

    // Tokenize and analyze code
    //let tokens = tokenize(&source_code);
    //let ast = parse(&tokens);

    // Displays the tokens and AST
    //println!("Tokens: {:?}", tokens);
    //println!("AST: {:?}", ast);

    //Ok(())

    // ! Creating a cleaner output
    let source_code = fs::read_to_string("examples/examples.xylon")?;

    let tokens = tokenize(&source_code);
    let ast = parse(&tokens);

    println!("Tokens: {:?}", tokens);

    println!("\nFormatted AST:");

    for expr in &ast {
        println!("{}", expr);
    }

    Ok(())
}
