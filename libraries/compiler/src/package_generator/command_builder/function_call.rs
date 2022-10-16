use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::package_generator::utils::{combine_command, jump_command_address_placeholder};
use crate::shared::ast::action::CallAction;
use crate::shared::command_map::{FunctionCommand, RootCommand};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationTarget, RelocationTargetElement};

pub fn build_function_call_command(
    action: &CallAction,
    defined_data: &Vec<DataDeclarator>,
    metadata: &PackageMetadata,
) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();

    // Build all parameters
    let mut sorted_params = action.arguments.clone();
    sorted_params.reverse();
    for expr in &sorted_params {
        let expr_eval_cmd = build_expression_evaluation_command(expr, defined_data, metadata);
        result.combine(expr_eval_cmd);
    }

    result.command_entries.push(result.commands.len());

    result.descriptors.targets.push(RelocationTarget {
        relocation_elements: vec![RelocationTargetElement::EnterFunction(action.function_name.clone())],
        command_array_position: result.commands.len(),
        offset: 1,
        relocated_address: 0,
    });

    result.append_commands(vec![combine_command(RootCommand::Function.to_opcode(), FunctionCommand::Enter.to_opcode())]);

    result.append_commands(jump_command_address_placeholder(metadata));
    result.append_commands(vec![sorted_params.len() as u8]);

    return result;
}
