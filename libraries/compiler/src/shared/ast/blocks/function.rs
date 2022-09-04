use crate::shared::ast::action::Action;
use crate::shared::ast::parameter::Parameter;
use crate::shared::utils::identifier::Identifier;

#[derive(Clone, Debug)]
pub struct Function {
    pub declarator: FunctionDeclarator,
    pub body: Vec<Action>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclarator {
    pub identifier: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Identifier,
}
