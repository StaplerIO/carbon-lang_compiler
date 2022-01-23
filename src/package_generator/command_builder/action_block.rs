use apa::apa::addition::add;

use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::package_generator::command_builder::data_commands::build_data_declaration_command;
use crate::package_generator::command_builder::expression_evaluation::convert_number_to_hex;
use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::package_generator::utils::{align_data_width, convert_to_u8_array};
use crate::shared::ast::action::{ActionBlock, ActionType};
use crate::shared::package_generation::data_descriptor::DataDeclaration;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

pub fn action_block_builder(block: &ActionBlock, metadata: &PackageMetadata) -> Vec<u8> {
    let mut result = vec![];

    let mut defined_data: Vec<DataDeclaration> = vec![];
    let mut data_count = String::from("0");

    for action in &block.actions {
        match action.action_type {
            ActionType::DeclarationStatement => {
                result.extend(build_data_declaration_command(false));
                defined_data.push(DataDeclaration {
                    name: action.clone().declaration_action.unwrap().identifier,
                    slot: align_data_width(
                        convert_to_u8_array(convert_number_to_hex(data_count.clone())),
                        metadata.data_alignment,
                    ),
                    is_global: false,
                });

                data_count = add(data_count.clone(), String::from("1"));
            }
            ActionType::AssignmentStatement => {
                result.extend(build_assignment_command(
                    &action.clone().assignment_action.unwrap(),
                    &defined_data,
                    metadata,
                ));
            }
            ActionType::CallStatement => {
                result.extend(build_function_call_command(
                    &action.clone().call_action.unwrap(),
                    &defined_data,
                    metadata,
                    &vec![],
                ));
            }
            ActionType::ReturnStatement => {
                // Will jump to destroy the domain
            }
            ActionType::IfStatement => {}
            ActionType::WhileStatement => {}
            ActionType::BreakStatement => {}
            ActionType::ContinueStatement => {}
            _ => {
                panic!("Command not supported!")
            }
        }
    }

    return result;
}
