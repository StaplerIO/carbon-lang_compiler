use crate::shared::ast::blocks::action::ActionBlock;
use crate::shared::ast::parameter::Parameter;

pub struct Function {
    pub name: String,

    pub parameters: Vec<Parameter>,
    pub body: ActionBlock,
    pub return_type: String
}
