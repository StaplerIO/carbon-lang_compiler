use crate::shared::ast::action::CallAction;
use crate::shared::ast::decorated_token::{DataToken, DecoratedToken};
use crate::shared::token::Operator;

#[derive(Clone)]
pub struct Expression {
    pub postfix_expr: Vec<ExprTerm>
}

#[derive(Clone, PartialEq, Debug)]
pub enum TermType {
    Data,
    FunctionCall,
    Operator,
    Priority,
}

#[derive(Clone)]
pub struct ExprTerm {
    pub term_type: TermType,

    pub data: Option<DataToken>,
    pub function_call: Option<CallAction>,
    pub operator: Option<Operator>,
    // true -> increase priority | false -> decrease priority
    pub priority: Option<bool>,
}
