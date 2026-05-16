use crate::ast::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Statements {
    Expression(Expr),
    VariablesDeclaration { name: String, intializer: Expr },
}
