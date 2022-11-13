use crate::shared::ast::group::implementation::GroupImplementationBlock;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;
use crate::shared::utils::identifier::Identifier;

pub type GroupTable = Vec<GroupTableEntry>;

#[derive(Clone, Debug)]
pub struct GroupTableEntry {
    pub slot: usize,
    pub identifier: Identifier,
    pub fields: Vec<FieldEntry>,
    pub relocated_method: Vec<FunctionEntry>,
    pub relocated_function: Vec<FunctionEntry>,

    pub implementation_table: Vec<GroupImplementationBlock>
}

#[derive(Clone, Debug)]
pub struct FieldEntry {
    pub type_id: Identifier,
    pub name: Identifier,
}

#[derive(Clone, Debug)]
pub struct FunctionEntry {
    pub identifier: Identifier,
    pub implementations_entry: Vec<PendingFunctionImplementation>,
    pub return_type_id: Identifier,
}

#[derive(Clone, Debug)]
pub struct PendingFunctionImplementation {
    pub entry_address: usize,
    pub commands: RelocatableCommandList
}

pub struct GeneratedGroup {
    pub identifier: Identifier,
    pub slot: usize,
}
