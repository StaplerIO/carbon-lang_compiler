use crate::shared::ast::action::CallAction;
use crate::shared::token::Operator;

#[derive(Clone)]
pub struct Expression {
    pub postfix_expr: Vec<ExprTerm>,

    pub output_type: String
}

#[derive(Clone, PartialEq, Debug)]
pub enum TermType {
    Data,
    Operator,
    Priority,

    // Only available in compile time
    Validated
}

#[derive(Clone)]
pub struct ExprTerm {
    pub term_type: TermType,

    pub data: Option<ExprDataTerm>,
    pub operator: Option<Operator>,
    // true -> increase priority | false -> decrease priority
    pub priority: Option<bool>,
}

#[derive(Clone)]
pub struct ExprDataTerm {
    pub data_type: ExprDataTermType,
    pub number: Option<String>,
    pub string: Option<String>,
    pub identifier: Option<String>,
    pub function_call: Option<CallAction>,

    // The data type of current term
    pub type_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprDataTermType {
    Number,
    String,
    Identifier,
    FunctionCall,
    TypeName,
}
