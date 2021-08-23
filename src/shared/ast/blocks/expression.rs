use crate::shared::ast::action::CallAction;
use crate::shared::ast::decorated_token::DataToken;
use crate::shared::token::Operator;

#[derive(Clone)]
pub struct Expression {
    pub postfix_expr: Vec<ExprTerm>,

    pub output_type: String
}

#[derive(Clone, PartialEq, Debug)]
pub enum TermType {
    Data,
    FunctionCall,
    Operator,
    Priority,

    // Only available in compile time
    Validated
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
