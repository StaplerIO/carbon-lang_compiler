use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::package_generator::command_builder::math::calculation::{inverse_command, minus_command};
use crate::package_generator::utils::{align_data_width, combine_command, jump_command_address_placeholder};
use crate::shared::ast::action::ConditionBlock;
use crate::shared::ast::blocks::expression::RelationExpression;
use crate::shared::command_map::{JumpCommand, RootCommand, StackCommand};
use crate::shared::package_generation::data_descriptor::DataDeclaration;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_descriptor::{JumpCommandBuildResult, RelocationDescriptor, RelocationType};
use crate::shared::token::RelationOperator;

pub fn condition_block_command_builder(action: &ConditionBlock, domains_after: usize, cmd_offset: isize, defined_data: &Vec<DataDeclaration>, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    let mut result = JumpCommandBuildResult {
        commands: vec![],
        descriptors: vec![]
    };

    let expr_eval_command = relation_expression_eval_command_builder(&action.condition, defined_data, metadata);
    // Build JumpCommand
    let mut jump_command: Vec<u8> = vec![];
    jump_command.push(combine_command(RootCommand::Jump.to_opcode(), JumpCommand::ByStackTop.to_opcode()));
    // Repeat placeholder for 3 times
    // Because in command *format manual*, there are 3 locations we need to assign
    jump_command.extend(jump_command_address_placeholder(metadata).repeat(3));
    // Add descriptors

    result.descriptors.extend(vec![
        RelocationDescriptor {
            relocation_type: RelocationType::IgnoreDomain(domains_after),
            offset: cmd_offset,
            command_array_position: 1,
            relocated_address: vec![]
        },
        RelocationDescriptor {
            relocation_type: RelocationType::IgnoreDomain(domains_after),
            command_array_position: metadata.address_alignment as usize,
            offset: cmd_offset,
            relocated_address: vec![]
        },
        RelocationDescriptor {
            relocation_type: RelocationType::IgnoreDomain(domains_after),
            offset: cmd_offset,
            command_array_position: (metadata.address_alignment as usize) * 2,
            relocated_address: vec![]
        }
    ]);

    // Modify target jump command
    match action.condition.expected_relation {
        RelationOperator::Bigger => {
            result.descriptors[0].relocation_type = RelocationType::NextCommand;
            result.descriptors[0].offset = 0;
        }
        RelationOperator::BiggerEqual => {
            result.descriptors[0].relocation_type = RelocationType::NextCommand;
            result.descriptors[0].offset = 0;
        }
        RelationOperator::Less => {
            result.descriptors[0].relocation_type = RelocationType::NextCommand;
            result.descriptors[0].offset = 0;
        }
        RelationOperator::LessEqual => {
            result.descriptors[0].relocation_type = RelocationType::NextCommand;
            result.descriptors[0].offset = 0;
        }
        RelationOperator::NotEqual => {
            result.descriptors[0].relocation_type = RelocationType::NextCommand;
            result.descriptors[0].offset = 0;

            result.descriptors[2].relocation_type = RelocationType::NextCommand;
            result.descriptors[2].offset = 0;
        }
        RelationOperator::Equal => {
            result.descriptors[1].relocation_type = RelocationType::NextCommand;
            result.descriptors[1].offset = 0;
        }
        _ => panic!("Illegal operator")
    };

    result.commands.extend(expr_eval_command);
    result.commands.extend(jump_command);

    // Put True commands
    result.commands.extend(action_block_builder(&action.body, metadata));

    return result;
}

// Return value
// First value is the command array
fn relation_expression_eval_command_builder(expr: &RelationExpression, defined_data: &Vec<DataDeclaration>, metadata: &PackageMetadata) -> Vec<u8> {
    let mut result = vec![];

    // Evaluate expressions
    // The evaluation result of the left-side expression is on the 2nd top position
    // The right-side result is on the first position
    result.extend(build_expression_evaluation_command(&expr.left, defined_data, metadata));
    result.extend(build_expression_evaluation_command(&expr.right, defined_data, metadata));

    // How to express the relation?
    match expr.expected_relation {
        RelationOperator::Bigger => {
            result.extend(minus_command());
            return result;
        }
        RelationOperator::BiggerEqual => {
            result.extend(minus_command());
            result.extend(plus_one(metadata.data_alignment));

            return result;
        }
        RelationOperator::Less => {
            result.extend(minus_command());
            result.extend(inverse_command());

            return result;
        }
        RelationOperator::LessEqual => {
            result.extend(minus_command());
            result.extend(inverse_command());
            result.extend(plus_one(metadata.data_alignment));

            return result;
        }
        RelationOperator::NotEqual => {
            result.extend(minus_command());
            return result;
        }
        RelationOperator::Equal => {
            result.extend(minus_command());
            return result;
        }
        _ => panic!("Illegal operator")
    };
}

fn plus_one(width: u8) -> Vec<u8> {
    let mut result = vec![];

    result.push(combine_command(RootCommand::Stack.to_opcode(), StackCommand::Push.to_opcode()));
    result.extend(align_data_width(vec![0x01], width));

    return result;
}

pub fn direct_jump_command_builder(r_type: RelocationType, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    let mut cmd_arr = vec![];

    cmd_arr.push(combine_command(RootCommand::Jump.to_opcode(), JumpCommand::ToOffset.to_opcode()));
    cmd_arr.extend(jump_command_address_placeholder(metadata));

    return JumpCommandBuildResult {
        commands: cmd_arr,
        descriptors: vec![RelocationDescriptor {
            relocation_type: r_type,
            offset: 0,
            command_array_position: 1,
            relocated_address: vec![]
        }]
    };
}
