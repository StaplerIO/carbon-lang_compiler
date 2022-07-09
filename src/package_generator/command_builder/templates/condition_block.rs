use crate::package_generator::command_builder::templates::jump_command::jump_by_stack_top_command_template_builder;
use crate::shared::ast::action::ConditionBlock;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationTargetType};

pub fn condition_block_builder(block: &ConditionBlock, domains_after: usize, defined_data: &Vec<DataDeclarator>, metadata: &PackageMetadata) -> RelocatableCommandList {
    let mut remaining_domain_count = 1 + domains_after;

    let r1 = jump_by_stack_top_command_template_builder(&block.condition, defined_data, metadata);
    let mut reloc_list = r1.0;

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

    return reloc_list;
}
