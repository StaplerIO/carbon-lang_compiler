use crate::shared::ast::action::Action;
use crate::shared::ast::parameter::Parameter;

pub struct Function {
    pub name: String,

    pub parameters: Vec<Parameter>,
    pub body: Vec<Action>,
    pub return_type: String,
}
