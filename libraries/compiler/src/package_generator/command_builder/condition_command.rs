use crate::package_generator::command_builder::action_block::action_block_command_builder;
use crate::package_generator::command_builder::templates::condition_block::{condition_block_builder, ConditionBlockType};
use crate::package_generator::command_builder::templates::jump_command::direct_jump_command_builder;
use crate::shared::ast::action::{IfAction, WhileBlock};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationReference, RelocationReferenceType, RelocationTargetType};

pub fn if_command_builder(action: &IfAction,
                          defined_data: &Vec<DataDeclarator>,
                          metadata: &PackageMetadata
) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();
    let mut remaining_domain_count = action.elif_collection.len() + action.else_action.is_some() as usize;

    // Build IfBlock
    result.combine(condition_block_builder(&action.if_block, ConditionBlockType::IfBlock, remaining_domain_count, defined_data, metadata));
    // remaining_domain_count -= 1;

    // Check if there are Elif and Else blocks
    if remaining_domain_count > 0 {
        // Jump out of IfBlock if there are
        result.combine(direct_jump_command_builder(RelocationTargetType::IgnoreDomain(remaining_domain_count), metadata));
    }

    // Build ElifBlock
    for elif_block in &action.elif_collection {
        result.combine(condition_block_builder(elif_block, ConditionBlockType::ElifBlock, remaining_domain_count, defined_data, metadata));
        remaining_domain_count -= 1;
        result.combine(direct_jump_command_builder(RelocationTargetType::IgnoreDomain(remaining_domain_count), metadata));
    }

    if action.else_action.is_some() {
        // Pack ElseAction into a domain
        result.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::ElseEntrance, command_array_position: result.commands.len() });
        result.combine(action_block_command_builder(&action.else_action.as_ref().unwrap(), true, defined_data, metadata));
        result.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::EndElse, command_array_position: result.commands.len() });
    }

    return result;
}

pub fn while_command_builder(action: &WhileBlock,
                             defined_data: &Vec<DataDeclarator>,
                             metadata: &PackageMetadata
) -> RelocatableCommandList {
    let mut result = condition_block_builder(action, ConditionBlockType::WhileBlock, 0, defined_data, metadata);
    result.combine(direct_jump_command_builder(RelocationTargetType::Relative(0 - result.commands.len() as i32), metadata));

    // Make a copy first
    let mut targets_copy = result.descriptors.targets.clone();
    
    // Ignore the back-to-entrance jump command when condition is already unsatisfied
    for (index, item) in result.descriptors.targets.iter().enumerate() {
        match item.relocation_type {
            RelocationTargetType::IgnoreDomain(_) => {
                targets_copy[index].offset = 1 + metadata.address_alignment as i32;
            },
            _ => {}
        }
    }
    
    // Paste it back
    result.descriptors.targets = targets_copy;

    return result;
}
