use crate::shared::ast::action::ActionBlock;
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::ast::blocks::function::Function;
use crate::shared::utils::identifier::Identifier;

pub type MethodImplementation = Function;
pub type FunctionImplementation = Function;
pub type FieldGS = ActionBlock;

pub struct GroupImplementationBlock {
    pub source_group: Identifier,

    pub fields: Vec<FieldImplementation>,
    pub methods: Vec<MethodImplementation>,
    pub functions: Vec<FunctionImplementation>,
}

pub struct FieldImplementation {
    pub identifier: Identifier,

    pub get_block: Option<FieldGS>,
    pub set_block: Option<FieldGS>,

    pub default_value: SimpleExpression
}
