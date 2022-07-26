use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::package_generator::utils::{align_data_width, convert_to_u8_array};
use crate::shared::ast::action::ActionBlock;
use crate::shared::ast::blocks::function::Function;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationReference};
use crate::shared::package_generation::relocation_reference::RelocationReferenceType::{EndFunction, FunctionEntrance};

pub fn build_function_command(func: &Function, metadata: &PackageMetadata) -> RelocatableCommandList {
    let mut params: Vec<DataDeclarator> = vec![];
    for (index, param) in func.parameters.iter().enumerate() {
        params.push(DataDeclarator {
            name: param.identifier.clone(),
            slot: align_data_width(convert_to_u8_array(index.to_string()),
                                   metadata.data_alignment),
            is_global: false,
        });
    }

    let mut result = action_block_builder(&ActionBlock { actions: func.body.clone() }, true, &params, metadata);
    result.descriptors.references.push(RelocationReference { ref_type: FunctionEntrance, command_array_position: 0 });
    result.descriptors.references.push(RelocationReference { ref_type: EndFunction, command_array_position: result.commands.len() - 1 });

    return result;
}
