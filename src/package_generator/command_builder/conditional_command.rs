use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::package_generator::utils::{combine_command, jump_command_address_placeholder};
use crate::shared::ast::action::{ConditionBlock, IfAction};
use crate::shared::command_map::{JumpCommand, RootCommand};
use crate::shared::package_generation::data_descriptor::DataDeclaration;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_descriptor::{JumpCommandBuildResult, RelocationDescriptor, RelocationType};

pub fn if_command_builder(action: &IfAction, defined_data: &Vec<DataDeclaration>, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    let mut result = JumpCommandBuildResult {
        commands: vec![],
        descriptors: vec![]
    };

    // Build IfBlock
    let mut after_domains: usize = 1 + &action.elif_collection.len() + action.else_action.is_some() as usize;
    result.append(condition_block_builder(&action.if_block, after_domains, defined_data, metadata));

    // Push ElifBlocks
    for elif_block in &action.elif_collection {}

    return result;
}

fn condition_block_builder(action: &ConditionBlock, domains_after: usize, defined_data: &Vec<DataDeclaration>, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    let mut result = JumpCommandBuildResult {
        commands: vec![],
        descriptors: vec![]
    };

    let body = action.clone().body;
    let expr_eval_command = build_expression_evaluation_command(&action.condition, defined_data, metadata);
    // Build JumpCommand
    let mut jump_command: Vec<u8> = vec![];
    jump_command.push(combine_command(RootCommand::Jump.to_opcode(), JumpCommand::ByStackTop.to_opcode()));
    // Repeat placeholder for 3 times
    // Because in command *format manual*, there are 3 locations we need to assign
    jump_command.extend(jump_command_address_placeholder(metadata).repeat(3));
    // Add descriptors
    result.descriptors.push(RelocationDescriptor {
        relocation_type: RelocationType::NextCommand,
        command_array_position: 1,
        relocated_address: vec![]
    });

    // Push relocation descriptor
    result.descriptors.push(RelocationDescriptor {
        relocation_type: RelocationType::IgnoreDomain(domains_after),
        command_array_position: metadata.address_alignment as usize,
        relocated_address: vec![]
    });
    result.descriptors.push(RelocationDescriptor {
        relocation_type: RelocationType::IgnoreDomain(domains_after),
        command_array_position: (metadata.address_alignment as usize) * 2,
        relocated_address: vec![]
    });

    result.commands.extend(expr_eval_command);
    result.commands.extend(jump_command);

    return result;
}
