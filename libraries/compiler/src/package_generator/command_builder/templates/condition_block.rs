use crate::package_generator::command_builder::action_block::action_block_command_builder;
use crate::package_generator::command_builder::templates::jump_command::{direct_jump_command_builder, jump_by_stack_top_command_template_builder};
use crate::shared::ast::action::ConditionBlock;
use crate::shared::command_map::JumpCommand;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationReference, RelocationReferenceType, RelocationTargetElement};

#[allow(dead_code)]
pub enum ConditionBlockType {
    IfBlock,
    ElifBlock,
    WhileBlock,
    Bare,
}

pub fn condition_block_builder(block: &ConditionBlock,
                               c_type: ConditionBlockType,
                               domains_after: usize,
                               defined_data: &Vec<DataDeclarator>,
                               metadata: &PackageMetadata,
) -> RelocatableCommandList {
    let remaining_domain_count = 1 + domains_after;

    let jump_command = jump_by_stack_top_command_template_builder(&block.condition, defined_data, metadata);
    // let len = r1.0.commands.len();
    let mut reloc_list = jump_command.0;

    // When encountered commands such as `break` or `continue`, we need to reevaluate the expression
    match c_type {
        ConditionBlockType::IfBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::IfEntrance, command_array_position: 0 });
        }
        ConditionBlockType::ElifBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::ElifEntrance, command_array_position: 0 });
        }
        ConditionBlockType::WhileBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::WhileEntrance, command_array_position: 0 });
        }
        ConditionBlockType::Bare => {}
    }

    // Update relocation target by what we required
    let args = jump_command.1;
    if args.0 {
        reloc_list.descriptors.targets[0].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
    } else {
        reloc_list.descriptors.targets[0].relocation_elements = vec![RelocationTargetElement::IgnoreDomain(remaining_domain_count),
                                                                     RelocationTargetElement::Relative(JumpCommand::ToRelative.get_len(metadata.address_alignment) as i32)];
    }

    if args.1 {
        reloc_list.descriptors.targets[1].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
    } else {
        reloc_list.descriptors.targets[1].relocation_elements = vec![RelocationTargetElement::IgnoreDomain(remaining_domain_count),
                                                                     RelocationTargetElement::Relative(JumpCommand::ToRelative.get_len(metadata.address_alignment) as i32)];
    }

    if args.2 {
        reloc_list.descriptors.targets[2].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
    } else {
        reloc_list.descriptors.targets[2].relocation_elements = vec![RelocationTargetElement::IgnoreDomain(remaining_domain_count),
                                                                     RelocationTargetElement::Relative(JumpCommand::ToRelative.get_len(metadata.address_alignment) as i32)];
    }

    reloc_list.combine(action_block_command_builder(&block.body, true, defined_data, metadata));

    match c_type {
        ConditionBlockType::IfBlock => {
            // If there's elif and else block, then jump out of them
            if remaining_domain_count > 1 {
                reloc_list.combine(direct_jump_command_builder(vec![RelocationTargetElement::IgnoreDomain(remaining_domain_count - 1)], metadata));
            }
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::EndIf, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::ElifBlock => {
            reloc_list.combine(direct_jump_command_builder(vec![RelocationTargetElement::IgnoreDomain(remaining_domain_count - 1)], metadata));
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::EndElif, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::WhileBlock => {
            reloc_list.combine(direct_jump_command_builder(vec![RelocationTargetElement::IterationHead], metadata));
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::EndWhile, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::Bare => {}
    }

    return reloc_list;
}
