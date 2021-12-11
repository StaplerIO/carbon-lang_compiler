use crate::package_generator::command_builder::math::calculation::{divide_command, minus_command, mod_command, multiplication_command, plus_command};
use crate::package_generator::utils::{align_data_width, combine_command, convert_to_u8_array, string_to_hex_char};
use crate::shared::ast::blocks::expression::{ExprDataTermType, Expression, TermType};
use crate::shared::command_map::{PLACE_HOLDER, RootCommand, StackCommand};
use crate::shared::package_generation::data_descriptor::DataDeclaration;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::token::{CalculationOperator, Operator, OperatorType};
use apa::apa::modulo::modulo;

pub fn expression_command_set_builder(expr: Expression, defined_data: &Vec<DataDeclaration>, metadata: &PackageMetadata) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    for term in expr.postfix_expr {
        if term.term_type == TermType::Data {
            let data = term.data.unwrap();

            // push data to stack
            if data.data_type == ExprDataTermType::Identifier {
                let identifier = defined_data.iter().find(|&x| x.name == data.clone().identifier.unwrap()).unwrap();

                // Push leading command
                result.push(combine_command(RootCommand::Stack.to_opcode(), StackCommand::PushFromObject.to_opcode()));
                // Push GDF(visit src/shared/command_map.rs)
                result.push(combine_command(u8::from(identifier.is_global), PLACE_HOLDER));
                // Push slot id
                result.extend(identifier.slot.clone());
            } else if data.data_type == ExprDataTermType::Number {
                result.push(combine_command(RootCommand::Stack.to_opcode(), StackCommand::Push.to_opcode()));

                let hex = convert_number_to_hex(data.number.unwrap());
                let hex_array = convert_to_u8_array(hex);

                result.extend(align_data_width(hex_array, metadata.data_alignment));
            } else {
                panic!("Other types of ExprTerm are not implemented yet!");
            }
        } else if term.term_type == TermType::Operator {
            let operator = term.operator.unwrap();

            result.extend(operator_opcode_builder(&operator));
        }
    }

    return result;
}

pub fn operator_opcode_builder(operator: &Operator) -> Vec<u8> {
    return match operator.operator_type {
        OperatorType::Calculation => {
            match operator.calculation.unwrap() {
                CalculationOperator::Plus => plus_command(),
                CalculationOperator::Minus => minus_command(),
                CalculationOperator::Times => multiplication_command(),
                CalculationOperator::Divide => divide_command(),
                CalculationOperator::Mod => mod_command()
            }
        }
        _ => {
            panic!("`Logical` and `Relation` operators are not implemented so far :(");
        }
    };
}

pub fn convert_number_to_hex(mut number: String) -> String {
    // Means it is already a hex number
    if number.starts_with("0x") {
        number.remove(0);
        number.remove(0);

        return number;
    }

    // Otherwise, it is a decimal. Convert it to hex
    let mut result = String::new();
    while !number.eq("0") {
        let modulo_result = modulo(number.clone(), String::from("16"));

        result.push(string_to_hex_char(modulo_result.1));
        number = modulo_result.0;
    }

    result = result.chars().rev().collect();

    return result;
}
