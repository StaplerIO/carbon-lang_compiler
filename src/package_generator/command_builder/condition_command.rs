use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::package_generator::command_builder::templates::jump_command::{condition_block_command_builder, direct_jump_command_builder};
use crate::shared::ast::action::IfAction;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{JumpCommandBuildResult, RelocationTargetType};

pub fn if_command_builder(action: &IfAction, defined_data: &Vec<DataDeclarator>, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    let mut result = JumpCommandBuildResult {
        commands: vec![],
        descriptors: vec![]
    };

    // We are going to add a jump command right after each IfBlock, ElifBlock or ElseBlock
    // So if the condition doesn't match, we need to
    let offset: isize = 1 + metadata.address_alignment as isize;

    // Build IfBlock
    let mut domains_after: usize = &action.elif_collection.len() + action.else_action.is_some() as usize;
    result.append(condition_block_command_builder(&action.if_block, 1, offset, defined_data, metadata));
    result.append(direct_jump_command_builder(RelocationTargetType::IgnoreDomain(domains_after.clone()), metadata));

    // Push ElifBlocks
    for elif_block in &action.elif_collection {
        domains_after -= 1;
        result.append(condition_block_command_builder(elif_block, 1, offset, defined_data, metadata));
        result.append(direct_jump_command_builder(RelocationTargetType::IgnoreDomain(domains_after.clone()), metadata));
    }

    if action.else_action.is_some() {
        result.append(action_block_builder(&action.else_action.clone().unwrap(), metadata));
    }

    return result;
}
