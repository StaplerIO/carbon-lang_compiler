use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::package_generator::command_builder::condition_command::if_command_builder;
use crate::package_generator::command_builder::data_commands::build_data_declaration_command;
use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::package_generator::command_builder::loop_interception::{break_action_command_builder, continue_action_command_builder};
use crate::package_generator::utils::{align_data_width, convert_to_u8_array};
use crate::shared::ast::action::{ActionBlock, ActionContent};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::JumpCommandBuildResult;

type UnrelocatedCommand = JumpCommandBuildResult;

pub fn action_block_builder(block: &ActionBlock, metadata: &PackageMetadata) -> UnrelocatedCommand {
    let mut result = UnrelocatedCommand{ commands: vec![], descriptors: vec![] };

    let mut defined_data: Vec<DataDeclarator> = vec![];
    let mut data_count: usize = 0;

    for action in &block.actions {
        match &action.content {
            ActionContent::DeclarationStatement(x) => {
                result.append_commands(build_data_declaration_command(false));
                defined_data.push(DataDeclarator {
                    name: x.identifier.clone(),
                    slot: align_data_width(
                        convert_to_u8_array(format!("{:X}", data_count)),
                        metadata.data_alignment,
                    ),
                    is_global: false,
                });

                data_count += 1;
            }
            ActionContent::AssignmentStatement(x) => {
                result.append_commands(build_assignment_command(
                    &x,
                    &defined_data,
                    metadata,
                ));
            }
            ActionContent::CallStatement(x) => {
                result.append_commands(build_function_call_command(
                    &x,
                    &defined_data,
                    metadata,
                    &vec![],
                ));
            }
            ActionContent::ReturnStatement(_x) => {
                // Will jump to destroy the domain
            }
            ActionContent::IfBlock(x) => {
                result.append(if_command_builder(&x, &defined_data, &metadata));
            }
            ActionContent::WhileStatement(_) => {}
            ActionContent::BreakStatement => {
                result.append(break_action_command_builder(&metadata));
            }
            ActionContent::ContinueStatement => {
                result.append(continue_action_command_builder(&metadata));
            }
            _ => {
                panic!("Command not supported!")
            }
        }
    }

    return result;
}
