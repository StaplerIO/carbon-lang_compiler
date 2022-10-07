use crate::package_generator::command_builder::action_block::action_block_command_builder;
use crate::package_generator::command_builder::templates::condition_block::{condition_block_builder, ConditionBlockType};
use crate::package_generator::command_builder::templates::jump_command::{direct_jump_command_builder, jump_by_stack_top_command_template_builder};
use crate::shared::ast::action::{IfAction, WhileBlock};
use crate::shared::command_map::JumpCommand;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationReference, RelocationReferenceType, RelocationTargetElement};

pub fn if_command_builder(action: &IfAction,
                          defined_data: &Vec<DataDeclarator>,
                          metadata: &PackageMetadata,
) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();
    let mut remaining_domain_count = action.elif_collection.len() + action.else_action.is_some() as usize;

    // Build IfBlock
    result.combine(condition_block_builder(&action.if_block, ConditionBlockType::IfBlock, remaining_domain_count, defined_data, metadata));

    // Build ElifBlock
    for elif_block in &action.elif_collection {
        result.combine(condition_block_builder(elif_block, ConditionBlockType::ElifBlock, remaining_domain_count, defined_data, metadata));
        remaining_domain_count -= 1;
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
                             metadata: &PackageMetadata,
) -> RelocatableCommandList {
    // Evaluate the expression first
    let mut eval_jump = jump_by_stack_top_command_template_builder(&action.condition, defined_data, metadata);
    // Generate body commands
    let mut while_body = action_block_command_builder(&action.body, true, defined_data, metadata);
    while_body.descriptors.references.push(RelocationReference {
        ref_type: RelocationReferenceType::WhileEntrance,
        command_array_position: 0,
    });
    while_body.descriptors.references.push(RelocationReference {
        ref_type: RelocationReferenceType::EndWhile,
        command_array_position: while_body.commands.len(),
    });

    // Jump back to re-evaluate the expression, judge if the condition is still satisfied
    let back_jump = direct_jump_command_builder(vec![RelocationTargetElement::Relative(-((eval_jump.0.commands.len() + while_body.commands.len()) as i32))], metadata);

    // Modify relocation elements from expression evaluation result
    let args = &eval_jump.1;
    if args.0 {
        // Jump to next instruction
        eval_jump.0.descriptors.targets[0].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
    } else {
        // Jump out of the whole statement
        eval_jump.0.descriptors.targets[0].relocation_elements = vec![RelocationTargetElement::IgnoreDomain(1),
                                                                      RelocationTargetElement::Relative(JumpCommand::ToRelative.get_len(metadata.address_alignment) as i32)];
    }

    if args.1 {
        eval_jump.0.descriptors.targets[1].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
    } else {
        eval_jump.0.descriptors.targets[1].relocation_elements = vec![RelocationTargetElement::IgnoreDomain(1),
                                                                      RelocationTargetElement::Relative(JumpCommand::ToRelative.get_len(metadata.address_alignment) as i32)];
    }

    if args.2 {
        eval_jump.0.descriptors.targets[2].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
    } else {
        eval_jump.0.descriptors.targets[2].relocation_elements = vec![RelocationTargetElement::IgnoreDomain(1),
                                                                      RelocationTargetElement::Relative(JumpCommand::ToRelative.get_len(metadata.address_alignment) as i32)];
    }

    // Combine command sections
    let mut result = RelocatableCommandList::new();
    result.combine(eval_jump.0);
    result.combine(while_body);
    result.combine(back_jump);

    return result;
}
