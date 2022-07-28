use crate::package_generator::utils::combine_command;
use crate::shared::command_map::{ObjectCommand, RootCommand, PLACE_HOLDER};
use crate::shared::package_generation::data_descriptor::DataAccessDescriptor;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

// 0xA100 for a private data
// 0xA110 for a global data
pub fn build_data_declaration_command(is_global: bool) -> RelocatableCommandList {
    return RelocatableCommandList::new_no_relocation(vec![
        combine_command(
            RootCommand::Object.to_opcode(),
            ObjectCommand::Create.to_opcode(),
        ),
        combine_command(u8::from(is_global), PLACE_HOLDER),
    ]);
}

pub fn data_access_command_builder(dac_model: &DataAccessDescriptor) -> Vec<u8> {
    let mut result = vec![];

    result.push(combine_command(u8::from(dac_model.is_global), PLACE_HOLDER));

    if !dac_model.is_global {
        result.extend(dac_model.domain_layer.clone());
    }

    result.extend(dac_model.slot.clone());

    return result;
}
