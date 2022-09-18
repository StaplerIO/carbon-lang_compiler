use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::package_generator::utils::combine_command;
use crate::shared::ast::action::ReturnAction;
use crate::shared::command_map::{FunctionCommand, RootCommand};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

pub fn return_command_builder(action: &ReturnAction, defined_data: &Vec<DataDeclarator>, metadata: &PackageMetadata) -> RelocatableCommandList {
    if action.value.is_some() {
        // Return from function with value
        let mut result = build_expression_evaluation_command(&action.value.clone().unwrap(), defined_data, metadata);
        if !result.commands.is_empty() {
            result.append_commands(vec![combine_command(RootCommand::Function.to_opcode(), FunctionCommand::LeaveWithValue.to_opcode())]);
            return result;
        }
    }

    return RelocatableCommandList::new_no_relocation(vec![combine_command(RootCommand::Function.to_opcode(), FunctionCommand::LeaveWithoutValue.to_opcode())]);
}
