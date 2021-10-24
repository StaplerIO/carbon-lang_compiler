use crate::package_generator::utils::combine_command;
use crate::shared::command_map::{RootCommand, ObjectCommand, PLACE_HOLDER};

// 0xA100 for a private data
// 0xA110 for a global data
pub fn data_declaration_builder(is_global: bool) -> Vec<u8> {
    return vec![combine_command(RootCommand::Object.to_opcode(), ObjectCommand::Create.to_opcode()),
                combine_command(u8::from(is_global), PLACE_HOLDER)];
}
