use crate::shared::ast::blocks::expression::Expression;

pub struct Parameter {
    pub type_name: String,
    pub identifier: String
}

pub struct Argument {
    pub identifier: String,
    pub value: Expression
}
