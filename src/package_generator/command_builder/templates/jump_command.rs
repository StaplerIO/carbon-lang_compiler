use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::package_generator::command_builder::math::calculation::minus_command;
use crate::package_generator::utils::{combine_command, jump_command_address_placeholder};
use crate::shared::ast::blocks::expression::RelationExpression;
use crate::shared::command_map::{JumpCommand, RootCommand};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationCredential, RelocationTarget, RelocationTargetType};
use crate::shared::token::operator::RelationOperator;

/// ### Return value
/// - Index `0`: An uncompleted relocation descriptor
/// - Index `1`: The relocation target to be jumped when expression result could be True
///
pub fn jump_by_stack_top_command_template_builder(expr: &RelationExpression, defined_data: &Vec<DataDeclarator>, metadata: &PackageMetadata) -> (RelocatableCommandList, (bool, bool, bool)) {
    let mut result = RelocatableCommandList::new();

    // Evaluate expressions
    // The evaluation result of the left-side expression is on the 2nd top position
    // The right-side result is on the first position
    result.commands.extend(build_expression_evaluation_command(&expr.left, defined_data, metadata));
    result.commands.extend(build_expression_evaluation_command(&expr.right, defined_data, metadata));

    // Current stack layout
    // |      left        |
    // |      right       |
    // |      other       |

    result.commands.extend(minus_command());

    // Current stack layout
    // |   right - left   |
    // |      other       |

    result.append_commands(vec![combine_command(RootCommand::Jump.to_opcode(), JumpCommand::ByStackTop.to_opcode())]);

    // Pre-save relocation target
    // Add descriptors
    result.descriptors.targets.extend(vec![
        RelocationTarget {
            relocation_type: RelocationTargetType::Undefined,
            command_array_position: 1 + result.commands.len(),
            relocated_address: vec![],
        },
        RelocationTarget {
            relocation_type: RelocationTargetType::Undefined,
            command_array_position: metadata.address_alignment as usize + result.commands.len(),
            relocated_address: vec![],
        },
        RelocationTarget {
            relocation_type: RelocationTargetType::Undefined,

            command_array_position: (metadata.address_alignment as usize) * 2 + result.commands.len(),
            relocated_address: vec![],
        },
    ]);

    // Push placeholder
    result.append_commands(vec![0; metadata.data_alignment as usize * 3]);

    // Command scheme: `0xD2 <PositiveLocation(0)> <NegativePosition(1)> <ZeroPosition(2)>`
    let mut true_pos = (false, false, false);
    match expr.expected_relation {
        RelationOperator::Greater => {
            // left - right < 0
            true_pos.1 = true;
        }
        RelationOperator::GreaterOrEqual => {
            // left - right <= 0
            true_pos.1 = true;
            true_pos.2 = true;
        }
        RelationOperator::Less => {
            // left - right > 0
            true_pos.0 = true;
        }
        RelationOperator::LessOrEqual => {
            // left - right >= 0
            true_pos.0 = true;
            true_pos.2 = true;
        }
        RelationOperator::NotEqual => {
            // left - right != 0
            true_pos.0 = true;
            true_pos.1 = true;
        }
        RelationOperator::Equal => {
            // left - right == 0
            true_pos.2 = true;
        }
        _ => panic!("Illegal operator")
    };

    return (result, true_pos);
}

pub fn direct_jump_command_builder(r_type: RelocationTargetType, metadata: &PackageMetadata) -> RelocatableCommandList {
    let mut cmd_arr = vec![];

    cmd_arr.push(combine_command(RootCommand::Jump.to_opcode(), JumpCommand::ToAbsolute.to_opcode()));
    cmd_arr.extend(jump_command_address_placeholder(metadata));

    return RelocatableCommandList {
        commands: cmd_arr,
        descriptors: RelocationCredential {
            targets: vec![RelocationTarget {
                relocation_type: r_type,
                command_array_position: 1,
                relocated_address: vec![],
            }],
            references: vec![],
        },
        command_entries: vec![0],
    };
}
