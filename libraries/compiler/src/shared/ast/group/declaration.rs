use crate::shared::ast::blocks::function::FunctionDeclarator;
use crate::shared::utils::identifier::Identifier;

pub type MethodDeclarator = FunctionDeclarator;

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub identifier: Identifier,
    pub data_type: Identifier,

    pub has_get: bool,
    pub has_set: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupDeclarationBlock {
    pub identifier: Identifier,
    pub fields: Vec<Field>,
    pub methods: Vec<MethodDeclarator>,
    pub functions: Vec<FunctionDeclarator>
}
