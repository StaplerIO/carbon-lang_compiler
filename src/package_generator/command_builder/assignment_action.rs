use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::package_generator::utils::combine_command;
use crate::shared::ast::action::AssignmentAction;
use crate::shared::command_map::{PLACE_HOLDER, RootCommand, StackCommand};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::function::FunctionDescriptor;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

pub fn build_assignment_command(
    action: &AssignmentAction,
    defined_functions: &Vec<FunctionDescriptor>,
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
        build_expression_evaluation_command(&action.eval_expression, defined_functions, defined_data, metadata);
    result.combine(expression_command_set);

    // Push stack top to target data slot
    // Push leading command
    result.append_commands(vec![combine_command(
        RootCommand::Stack.to_opcode(),
        StackCommand::PopToObject.to_opcode(),
    )]);
    // Push GDF(visit src/shared/command_map.rs)
    result.append_commands(vec![combine_command(
        u8::from(target_data.is_global),
        PLACE_HOLDER,
    )]);
    // Push slot id
    result.append_commands(target_data.slot.clone());

    return result;
}
