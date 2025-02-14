// Definition of the AST structure

#[derive(Debug)]
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
pub enum Operator {
    Add, // addition (+)
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
}
