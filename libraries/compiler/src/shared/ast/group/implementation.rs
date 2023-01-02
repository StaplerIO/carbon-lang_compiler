use crate::shared::ast::action::ActionBlock;
use crate::shared::ast::blocks::data::DataDeclarator;
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::ast::blocks::function::Function;
use crate::shared::utils::identifier::Identifier;

pub type MethodImplementation = Function;
pub type FunctionImplementation = Function;
pub type FieldGS = ActionBlock;

#[derive(Debug, Clone)]
pub struct GroupImplementationBlock {
    pub source_group: Identifier,

    pub fields: Vec<FieldImplementation>,
    pub methods: Vec<MethodImplementation>,
    pub functions: Vec<FunctionImplementation>,
}

#[derive(Debug, Clone)]
pub struct FieldImplementation {
    pub source: DataDeclarator,
    pub slot: usize,

    pub get_block: Option<FieldGS>,
    pub set_block: Option<FieldGS>,

    pub default_value: SimpleExpression
}
