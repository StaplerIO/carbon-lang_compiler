use crate::package_generator::command_builder::action_block::action_block_command_builder;
use crate::package_generator::command_builder::templates::jump_command::{direct_jump_command_builder, jump_by_stack_top_command_template_builder};
use crate::shared::ast::action::{ConditionBlock, IfAction, WhileBlock};
use crate::shared::command_map::JumpCommand;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationReference, RelocationReferenceType, RelocationTargetElement};

struct ConditionBlockStagingStructure {
    eval_jump: (RelocatableCommandList, (bool, bool, bool)),
    body: RelocatableCommandList,
    jump_out: RelocatableCommandList,
}

pub fn if_command_builder(action: &IfAction,
                          defined_data: &Vec<DataDeclarator>,
                          metadata: &PackageMetadata,
) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();

    let mut condition_blocks: Vec<ConditionBlock> = action.elif_collection.clone();
    condition_blocks.insert(0, action.if_block.clone());

    let else_block_build_result: Option<RelocatableCommandList>;
    if action.else_action.is_some() {
        let else_block = action.else_action.clone().unwrap();
        let mut body = action_block_command_builder(&else_block, true, defined_data, metadata);

        body.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::ElseEntrance, command_array_position: 0 });
        body.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::EndElse, command_array_position: body.commands.len() });

        else_block_build_result = Some(body);
    } else {
        else_block_build_result = None;
    }

    let mut domain_count: usize = condition_blocks.len() + action.else_action.is_some() as usize;

    let mut reloc_sections = vec![];
    for block in &condition_blocks {
        let eval_jump = jump_by_stack_top_command_template_builder(&block.condition, defined_data, metadata);
        let body = action_block_command_builder(&block.body, true, defined_data, metadata);
        let jump_out = direct_jump_command_builder(vec![], metadata);

        let mut current_build = ConditionBlockStagingStructure {
            eval_jump,
            body,
            jump_out,
        };

        // Command layout:
        // [if/elif entrance]
        // <expression evaluation>
        // <if/elif body>
        // <jump to end of block>
        // [if/elif end]
        let args = &current_build.eval_jump.1;
        if args.0 {
            // Jump to next instruction
            current_build.eval_jump.0.descriptors.targets[0].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
        } else {
            // Jump out of the whole statement
            current_build.eval_jump.0.descriptors.targets[0].relocation_elements = vec![RelocationTargetElement::BreakDomain(1)];
        }

        if args.1 {
            current_build.eval_jump.0.descriptors.targets[1].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
        } else {
            current_build.eval_jump.0.descriptors.targets[1].relocation_elements = vec![RelocationTargetElement::BreakDomain(1)];
        }

        if args.2 {
            current_build.eval_jump.0.descriptors.targets[2].relocation_elements = vec![RelocationTargetElement::Relative(JumpCommand::ByStackTop.get_len(metadata.address_alignment) as i32)];
        } else {
            current_build.eval_jump.0.descriptors.targets[2].relocation_elements = vec![RelocationTargetElement::BreakDomain(1)];
        }

        if domain_count >= 1 {
            domain_count -= 1;
        }

        current_build.jump_out.descriptors.targets[0].relocation_elements.push(RelocationTargetElement::IgnoreDomain(domain_count));

        reloc_sections.push(current_build);
    }

    // If there's no else block, then remove the last jump_out
    if else_block_build_result.is_none() {
        let last_build = reloc_sections.last_mut().unwrap();
        last_build.jump_out = RelocatableCommandList::new();
    }

    let len = reloc_sections.len();

    // Calculate the full length of the IfBlock command
    let mut total_len: usize = 0;
    for (idx, section) in reloc_sections.iter().enumerate() {
        total_len += section.eval_jump.0.commands.len() + section.body.commands.len() + section.jump_out.commands.len();

        // Push else block or remove last jump out
        if idx == len - 1 {
            if else_block_build_result.is_none() {
                total_len -= section.jump_out.commands.len();
            } else {
                total_len += else_block_build_result.clone().unwrap().commands.len();
            }
        }
    }

    for (idx, section) in reloc_sections.iter().enumerate() {
        let mut current = RelocatableCommandList::new();
        current.combine(section.eval_jump.0.clone());
        current.combine(section.body.clone());

        // Generate references
        if idx == 0 {
            current.combine(section.jump_out.clone());

            current.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::IfEntrance, command_array_position: 0 });
            current.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::EndIf, command_array_position: current.commands.len() });

        } else {
            if else_block_build_result.is_none() && idx == len - 1 {
                // Modify the last one if there's no else branch
                current.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::ElifEntrance, command_array_position: 0 });
                current.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::EndElif, command_array_position: current.commands.len() });

                continue;
            } else {
                current.combine(section.jump_out.clone());

                current.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::ElifEntrance, command_array_position: 0 });
                current.descriptors.references.push(RelocationReference{ ref_type: RelocationReferenceType::EndElif, command_array_position: current.commands.len() });
            }
        }

        result.combine(current);

        // We are not capable to jump out directly from the IfBlock, so we need to calculate the relative to the end manually
        let last_reloc_target = result.descriptors.targets.last_mut().unwrap();
        last_reloc_target.relocation_elements[0] = RelocationTargetElement::Relative((total_len - last_reloc_target.command_array_position) as i32);
    }

    if else_block_build_result.is_some() {
        result.combine(else_block_build_result.unwrap());
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
