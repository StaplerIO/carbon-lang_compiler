use crate::shared::ast::decorated_token::{DecoratedToken, DataToken};
use crate::shared::ast::action::CallAction;
use crate::shared::token::Operator;

pub struct Expression {
    pub postfix_expr: Vec<DecoratedToken>
}

pub enum TermType {
    Data,
    FunctionCall,
    Operator
}

pub struct ExprTerm {
    pub term_type: TermType,

    pub data: Option<DataToken>,
    pub function_call: Option<CallAction>,
    pub operator: Option<Operator>
}
