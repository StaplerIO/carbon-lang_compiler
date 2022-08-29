use crate::shared::ast::action::CallAction;
use crate::shared::package_generation::data_descriptor::StringConstant;
use crate::shared::token::operator::{Operator, RelationOperator};
use crate::shared::token::token::Token;
use crate::shared::utils::identifier::Identifier;

#[derive(Clone, PartialEq, Debug)]
pub struct SimpleExpression {
    pub postfix_expr: Vec<ExprTerm>,

    pub output_type: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RelationExpression {
    pub left: SimpleExpression,
    pub right: SimpleExpression,

    pub expected_relation: RelationOperator
}

#[derive(Clone, PartialEq, Debug)]
pub enum TermContent {
    Data(ExprDataTerm),
    Operator(Operator),
    Priority(bool),

    // Only available in compile time
    Validated,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ExprTerm {
    pub content: TermContent,
    pub original_token: Vec<Token>
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprDataTerm {
    Number(String),
    String(StringConstant),
    Identifier(Identifier),
    FunctionCall(CallAction),
}
