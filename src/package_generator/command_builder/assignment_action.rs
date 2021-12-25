use crate::package_generator::command_builder::expression_evaluation::expression_command_set_builder;
use crate::package_generator::utils::combine_command;
use crate::shared::ast::action::AssignmentAction;
use crate::shared::command_map::{RootCommand, StackCommand, PLACE_HOLDER};
use crate::shared::package_generation::data_descriptor::DataDeclaration;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

pub fn build_assignment_command(
    action: &AssignmentAction,
    defined_data: &Vec<DataDeclaration>,
    metadata: &PackageMetadata,
) -> Vec<u8> {
    let mut result = vec![];
    let target_data = defined_data
        .iter()
        .find(|&x| x.name == action.identifier)
        .unwrap()
        .clone();

    let expression_command_set =
        expression_command_set_builder(&action.eval_expression, defined_data, metadata);
    result.extend(expression_command_set);

    // Push stack top to target data slot
    // Push leading command
    result.push(combine_command(
        RootCommand::Stack.to_opcode(),
        StackCommand::PopToObject.to_opcode(),
    ));
    // Push GDF(visit src/shared/command_map.rs)
    result.push(combine_command(
        u8::from(target_data.is_global),
        PLACE_HOLDER,
    ));
    // Push slot id
    result.extend(target_data.slot.clone());

    return result;
}
