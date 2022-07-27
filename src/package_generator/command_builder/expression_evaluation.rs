use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::package_generator::command_builder::math::calculation::{
    divide_command, minus_command, mod_command, multiplication_command, plus_command,
};
use crate::package_generator::utils::{align_data_width, combine_command, convert_number_to_hex, convert_to_u8_array};
use crate::shared::ast::blocks::expression::{ExprDataTerm, SimpleExpression};
use crate::shared::command_map::{RootCommand, StackCommand, PLACE_HOLDER};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::function::FunctionDescriptor;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;
use crate::shared::token::operator::{CalculationOperator, Operator};

// TODO: Mark commands by `result.command_entries.push(result.commands.len());`

/// The result of the expression is on the top of the `DomainStack`
pub fn build_expression_evaluation_command(
    expr: &SimpleExpression,
    defined_functions: &Vec<FunctionDescriptor>,
    defined_data: &Vec<DataDeclarator>,
    metadata: &PackageMetadata,
) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();

    for term in &expr.postfix_expr {
        if term.content.get_data_term().is_some() {
            let data = term.content.get_data_term().unwrap();

            // push data to stack
            match data {
                ExprDataTerm::Identifier(x) => {
                    // Seek existing identifiers
                    let identifier = defined_data
                        .iter()
                        .find(|&dd| dd.name == *x)
                        .unwrap();

                    // Push leading command
                    result.append_commands(vec![combine_command(
                        RootCommand::Stack.to_opcode(),
                        StackCommand::PushFromObject.to_opcode(),
                    )]);
                    // Push GDF(visit src/shared/command_map.rs)
                    result.append_commands(vec![combine_command(
                        u8::from(identifier.is_global),
                        PLACE_HOLDER,
                    )]);
                    // Push slot id
                    result.append_commands(identifier.slot.clone());
                },
                ExprDataTerm::Number(x) => {
                    result.append_commands(vec![combine_command(
                        RootCommand::Stack.to_opcode(),
                        StackCommand::Push.to_opcode(),
                    )]);

                    let number_hex = convert_number_to_hex(x.clone());
                    let number_hex_array = convert_to_u8_array(number_hex);

                    result.append_commands(align_data_width(number_hex_array, metadata.data_alignment));
                },
                ExprDataTerm::FunctionCall(x) => {
                    // The called function will automatically put the return value on the top of the stack
                    result.combine(build_function_call_command(x, defined_functions, defined_data, metadata));
                }
                _ => panic!("Other types of ExprTerm are not implemented yet!")
            }
        } else if term.content.get_operator().is_some() {
            let operator = term.content.get_operator().unwrap();
            result.append_commands(operator_opcode_builder(&operator));
        }
    }

    return result;
}

pub fn operator_opcode_builder(operator: &Operator) -> Vec<u8> {
    return match operator {
        Operator::Calculation(x) => match x {
            CalculationOperator::Addition => plus_command(),
            CalculationOperator::Subtraction => minus_command(),
            CalculationOperator::Multiply => multiplication_command(),
            CalculationOperator::Division => divide_command(),
            CalculationOperator::Modulo => mod_command(),
            _ => panic!("Invalid calculation operator"),
        },
        _ => {
            panic!("`Logical` and `Relation` operators are not implemented so far :(");
        }
    };
}
