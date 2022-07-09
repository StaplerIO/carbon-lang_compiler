use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::package_generator::command_builder::templates::condition_block::condition_block_builder;
use crate::shared::ast::action::IfAction;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

pub fn if_command_builder(action: &IfAction, defined_data: &Vec<DataDeclarator>, metadata: &PackageMetadata) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();
    let mut remaining_domain_count = action.elif_collection.len() + action.else_action.is_some() as usize;

    // Build IfBlock
    result.combine(condition_block_builder(&action.if_block, remaining_domain_count, defined_data, metadata));
    remaining_domain_count -= 1;

    // Build ElifBlock
    for elif_block in action.elif_collection {
        result.combine(condition_block_builder(&elif_block, remaining_domain_count, defined_data, metadata));
        remaining_domain_count -= 1;
    }

    if action.else_action.is_some() {
        // Pack ElseAction into a domain
        result.combine(action_block_builder(&action.else_action.unwrap(), true, defined_data, metadata));
    }

    return result;
}
