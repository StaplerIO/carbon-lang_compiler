use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::package_generator::command_builder::templates::jump_command::jump_by_stack_top_command_template_builder;
use crate::shared::ast::action::ConditionBlock;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationReference, RelocationReferenceType, RelocationTargetType};

pub enum ConditionBlockType {
    IfBlock,
    ElifBlock,
    WhileBlock,
    Bare,
}

pub fn condition_block_builder(block: &ConditionBlock, c_type: ConditionBlockType, domains_after: usize, defined_data: &Vec<DataDeclarator>, metadata: &PackageMetadata) -> RelocatableCommandList {
    let remaining_domain_count = 1 + domains_after;

    let r1 = jump_by_stack_top_command_template_builder(&block.condition, defined_data, metadata);
    let mut reloc_list = r1.0;

    match c_type {
        ConditionBlockType::IfBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::IfEntrance, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::ElifBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::ElifEntrance, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::WhileBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::WhileEntrance, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::Bare => {}
    }

    // Update relocation target by what we required
    let args = r1.1;
    if args.0 {
        reloc_list.descriptors.targets[0].relocation_type = RelocationTargetType::NextCommand;
    } else {
        reloc_list.descriptors.targets[0].relocation_type = RelocationTargetType::IgnoreDomain(remaining_domain_count);
    }

    if args.1 {
        reloc_list.descriptors.targets[1].relocation_type = RelocationTargetType::NextCommand;
    } else {
        reloc_list.descriptors.targets[1].relocation_type = RelocationTargetType::IgnoreDomain(remaining_domain_count);
    }

    if args.2 {
        reloc_list.descriptors.targets[2].relocation_type = RelocationTargetType::NextCommand;
    } else {
        reloc_list.descriptors.targets[2].relocation_type = RelocationTargetType::IgnoreDomain(remaining_domain_count);
    }

    reloc_list.combine(action_block_builder(&block.body, true, defined_data, metadata));

    match c_type {
        ConditionBlockType::IfBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::EndIf, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::ElifBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::EndElif, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::WhileBlock => {
            reloc_list.descriptors.references.push(RelocationReference { ref_type: RelocationReferenceType::EndWhile, command_array_position: reloc_list.commands.len() });
        }
        ConditionBlockType::Bare => {}
    }

    return reloc_list;
}
