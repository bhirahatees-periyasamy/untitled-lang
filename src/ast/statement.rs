use crate::ast::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Statements {
    Empty,
    Expression(Expr),
    VariablesDeclaration {  name: String, initializer: Expr },
}


impl Statements {
    pub fn new() -> Self {
        Statements::Empty
    }
}