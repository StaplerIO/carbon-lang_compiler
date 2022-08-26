use crate::shared::ast::action::Action;
use crate::shared::ast::parameter::Parameter;
use crate::shared::utils::identifier::Identifier;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Identifier,

    pub parameters: Vec<Parameter>,
    pub body: Vec<Action>,
    pub return_type: Identifier,
}
