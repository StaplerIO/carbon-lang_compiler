use crate::package_generator::command_builder::action_block::action_block_command_builder;
use crate::shared::ast::action::ActionBlock;
use crate::shared::ast::blocks::function::Function;
use crate::shared::package_generation::data_descriptor::{DataDeclarator, DataLocation};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationReference};
use crate::shared::package_generation::relocation_reference::RelocationReferenceType::{EndFunction, FunctionEntrance};

pub fn build_function_command(func: &Function, metadata: &PackageMetadata) -> RelocatableCommandList {
    let mut params: Vec<DataDeclarator> = vec![];
    for (index, param) in func.declarator.parameters.iter().enumerate() {
        params.push(DataDeclarator {
            name: param.identifier.clone(),
            slot: index,
            location: DataLocation::Local,
            is_string: false
        });
    }

    let mut result = action_block_command_builder(&ActionBlock { actions: func.body.clone() }, true, &params, metadata);
    result.descriptors.references.push(RelocationReference { ref_type: FunctionEntrance(func.declarator.identifier.clone()), command_array_position: 0 });
    result.descriptors.references.push(RelocationReference { ref_type: EndFunction(func.declarator.identifier.clone()), command_array_position: result.commands.len() - 1 });

    return result;
}
