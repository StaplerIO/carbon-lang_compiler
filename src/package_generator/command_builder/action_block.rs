use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::package_generator::command_builder::condition_command::{
    if_command_builder, while_command_builder,
};
use crate::package_generator::command_builder::data_commands::build_data_declaration_command;
use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::package_generator::command_builder::loop_interception::{
    break_action_command_builder, continue_action_command_builder,
};
use crate::package_generator::command_builder::return_from_function::return_command_builder;
use crate::package_generator::utils::{align_data_width, combine_command, convert_to_u8_array};
use crate::shared::ast::action::{ActionBlock, ActionContent};
use crate::shared::command_map::{DomainCommand, RootCommand};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

pub fn action_block_command_builder(
    block: &ActionBlock,
    surround_domain: bool,
    defined_data: &Vec<DataDeclarator>,
    metadata: &PackageMetadata,
) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();

    let mut available_defined_data: Vec<DataDeclarator> = defined_data.clone();

    // Create a new domain if necessary
    if surround_domain {
        result.command_entries.push(result.commands.len());
        result.append_commands(vec![combine_command(
            RootCommand::Domain.to_opcode(),
            DomainCommand::Create.to_opcode(),
        )]);
    }

    for action in &block.actions {
        match &action.content {
            ActionContent::DeclarationStatement(x) => {
                result.command_entries.push(result.commands.len());
                result.combine(build_data_declaration_command(false));
                available_defined_data.push(DataDeclarator {
                    name: x.identifier.clone(),
                    slot: align_data_width(
                        convert_to_u8_array(available_defined_data.len().to_string()),
                        metadata.data_alignment,
                    ),
                    is_global: false,
                });
            }
            ActionContent::AssignmentStatement(x) => {
                result.command_entries.push(result.commands.len());
                result.combine(build_assignment_command(
                    &x,
                    &available_defined_data,
                    metadata,
                ));
            }
            ActionContent::CallStatement(x) => {
                result.command_entries.push(result.commands.len());
                result.combine(build_function_call_command(
                    &x,
                    &available_defined_data,
                    metadata,
                ));
            }
            ActionContent::ReturnStatement(x) => {
                // Interrupt function execution
                result.combine(return_command_builder(x, &available_defined_data, metadata));
            }
            ActionContent::IfBlock(x) => {
                result.combine(if_command_builder(
                    x,
                    &available_defined_data,
                    &metadata,
                ));
            }
            ActionContent::WhileStatement(x) => {
                result.combine(while_command_builder(
                    x,
                    &available_defined_data,
                    &metadata,
                ));
            }
            ActionContent::BreakStatement => {
                result.command_entries.push(result.commands.len());
                result.combine(break_action_command_builder(&metadata));
            }
            ActionContent::ContinueStatement => {
                result.command_entries.push(result.commands.len());
                result.combine(continue_action_command_builder(&metadata));
            }
            _ => {
                panic!("Command not supported!")
            }
        }
    }

    // Destroy the domain if created
    if surround_domain {
        result.command_entries.push(result.commands.len());
        result.append_commands(vec![combine_command(
            RootCommand::Domain.to_opcode(),
            DomainCommand::Destroy.to_opcode(),
        )]);
    }

    return result;
}
