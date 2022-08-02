use crate::package_generator::command_builder::allocators::mutable_data_alloc::dac_builder;
use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::package_generator::utils::combine_command;
use crate::shared::ast::action::AssignmentAction;
use crate::shared::command_map::{RootCommand, StackCommand};
use crate::shared::package_generation::data_descriptor::{DataAccessDescriptor, DataDeclarator};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

pub fn build_assignment_command(
    action: &AssignmentAction,
    defined_data: &Vec<DataDeclarator>,
    metadata: &PackageMetadata,
) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();
    let target_data = defined_data
        .iter()
        .find(|&x| x.name == action.identifier)
        .unwrap()
        .clone();

    let expression_command_set =
        build_expression_evaluation_command(&action.eval_expression, defined_data, metadata);
    result.combine(expression_command_set);

    // Push stack top to target data slot
    // Push leading command
    result.command_entries.push(result.commands.len());
    result.append_commands(vec![combine_command(
        RootCommand::Stack.to_opcode(),
        StackCommand::PopToObject.to_opcode(),
    )]);
    // Acquire DAC
    let dac_build_result = dac_builder(DataAccessDescriptor::new_identifier(target_data), metadata);
    if dac_build_result.is_ok() {
        result.combine(dac_build_result.unwrap());
    } else {
        panic!("Failed to build data access command for identifier: {}", action.identifier);
    }

    return result;
}
