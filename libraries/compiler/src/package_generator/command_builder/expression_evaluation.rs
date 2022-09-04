use crate::package_generator::command_builder::allocators::mutable_data_alloc::dac_builder;
use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::package_generator::command_builder::math::calculation::{
    divide_command, minus_command, mod_command, multiplication_command, plus_command,
};
use crate::package_generator::utils::combine_command;
use crate::shared::ast::blocks::expression::{ExprDataTerm, SimpleExpression};
use crate::shared::command_map::{RootCommand, StackCommand};
use crate::shared::package_generation::data_descriptor::{DataAccessDescriptor, DataDeclarator};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;
use crate::shared::token::operator::{CalculationOperator, Operator};

// TODO: Mark commands by `result.command_entries.push(result.commands.len());`

/// The result of the expression is on the top of the `DomainStack`
pub fn build_expression_evaluation_command(
    expr: &SimpleExpression,
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
                    result.command_entries.push(result.commands.len());
                    result.append_commands(vec![combine_command(
                        RootCommand::Stack.to_opcode(),
                        StackCommand::PushFromObject.to_opcode(),
                    )]);

                    // Seek existing identifiers
                    let identifier = defined_data
                        .iter()
                        .find(|&dd| dd.name == *x)
                        .unwrap();

                    let dac_build_result = dac_builder(DataAccessDescriptor::new_identifier(identifier.clone()), metadata);
                    if dac_build_result.is_ok() {
                        result.combine(dac_build_result.unwrap());
                    } else {
                        panic!("Failed to build data access command for identifier: {}", x);
                    }
                },
                ExprDataTerm::Number(x) => {
                    result.command_entries.push(result.commands.len());
                    result.append_commands(vec![combine_command(
                        RootCommand::Stack.to_opcode(),
                        StackCommand::Push.to_opcode(),
                    )]);

                    let dac_build_result = dac_builder(DataAccessDescriptor::new_instant_value(x.clone()), metadata);
                    if dac_build_result.is_ok() {
                        result.combine(dac_build_result.unwrap());
                    } else {
                        panic!("Failed to build data access command for number: {}", x);
                    }
                },
                ExprDataTerm::FunctionCall(x) => {
                    // The called function will automatically put the return value on the top of the stack
                    result.combine(build_function_call_command(x, defined_data, metadata));
                },
                ExprDataTerm::String(x) => {
                    result.command_entries.push(result.commands.len());
                    result.append_commands(vec![combine_command(
                        RootCommand::Stack.to_opcode(),
                        StackCommand::PushFromObject.to_opcode(),
                    )]);

                    let dac_build_result = dac_builder(DataAccessDescriptor::new_string_constant(x.clone()), metadata);
                    if dac_build_result.is_ok() {
                        result.combine(dac_build_result.unwrap());
                    } else {
                        panic!("Failed to build data access command for string: {}", x.value);
                    }
                }
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
