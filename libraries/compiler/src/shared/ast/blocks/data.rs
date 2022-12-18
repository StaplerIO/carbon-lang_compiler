use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::utils::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct DataDeclarator {
    pub data_type: Identifier,
    pub identifier: Identifier,
    pub is_array: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataAccessor {
    Singleton(Identifier),
    ArrayElement(ArrayElementAccessor),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayElementAccessor {
    pub identifier: Identifier,
    pub index_eval_expr: SimpleExpression,
}
