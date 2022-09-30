use crate::shared::utils::identifier::Identifier;

pub type FunctionTable = Vec<FunctionTableEntry>;

#[derive(Clone, Debug)]
pub struct FunctionTableEntry {
    pub slot: usize,
    pub name: Identifier,
    pub relocated_entry_address: usize
}
