use crate::shared::utils::identifier::Identifier;

pub struct FunctionDescriptor {
    pub identifier: Identifier,
    pub entry_point: Vec<u8>,
}

pub struct FunctionCallDescriptor {
    pub target_function_identifier: String,
    pub relocation_start_loc: usize,
}
