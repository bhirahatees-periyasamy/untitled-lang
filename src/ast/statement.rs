use crate::ast::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Statements {
    Expression(Expr),
    VariableDeclaration {  name: String, initializer: Expr },
}
