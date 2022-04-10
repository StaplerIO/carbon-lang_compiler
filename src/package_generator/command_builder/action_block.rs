use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::package_generator::command_builder::condition_command::if_command_builder;
use crate::package_generator::command_builder::data_commands::build_data_declaration_command;
use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::package_generator::command_builder::loop_interception::{break_action_command_builder, continue_action_command_builder};
use crate::package_generator::utils::{align_data_width, convert_to_u8_array};
use crate::shared::ast::action::{ActionBlock, ActionType};
use crate::shared::package_generation::data_descriptor::DataDeclaration;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_descriptor::JumpCommandBuildResult;

type UnrelocatedCommand = JumpCommandBuildResult;

pub fn action_block_builder(block: &ActionBlock, metadata: &PackageMetadata) -> UnrelocatedCommand {
    let mut result = UnrelocatedCommand{ commands: vec![], descriptors: vec![] };

    let mut defined_data: Vec<DataDeclaration> = vec![];
    let mut data_count: usize = 0;

    for action in &block.actions {
        match action.action_type {
            ActionType::DeclarationStatement => {
                result.append_commands(build_data_declaration_command(false));
                defined_data.push(DataDeclaration {
                    name: action.clone().declaration_action.unwrap().identifier,
                    slot: align_data_width(
                        convert_to_u8_array(format!("{:X}", data_count)),
                        metadata.data_alignment,
                    ),
                    is_global: false,
                });

                data_count += 1;
            }
            ActionType::AssignmentStatement => {
                result.append_commands(build_assignment_command(
                    &action.clone().assignment_action.unwrap(),
                    &defined_data,
                    metadata,
                ));
            }
            ActionType::CallStatement => {
                result.append_commands(build_function_call_command(
                    &action.clone().call_action.unwrap(),
                    &defined_data,
                    metadata,
                    &vec![],
                ));
            }
            ActionType::ReturnStatement => {
                // Will jump to destroy the domain
            }
            ActionType::IfStatement => {
                result.append(if_command_builder(&action.if_action.clone().unwrap(), &defined_data, &metadata));
            }
            ActionType::WhileStatement => {}
            ActionType::BreakStatement => {
                result.append(break_action_command_builder(&metadata));
            }
            ActionType::ContinueStatement => {
                result.append(continue_action_command_builder(&metadata));
            }
            _ => {
                panic!("Command not supported!")
            }
        }
    }

    return result;
}
