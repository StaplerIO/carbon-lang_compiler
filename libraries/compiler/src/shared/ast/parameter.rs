use crate::shared::utils::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub type_name: Identifier,
    pub identifier: Identifier,
}
