use crate::shared::ast::blocks::data::DataType;
use crate::shared::utils::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub type_name: DataType,
    pub identifier: Identifier,
}
