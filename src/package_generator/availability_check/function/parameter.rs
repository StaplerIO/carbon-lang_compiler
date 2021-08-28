use crate::shared::ast::action::{CallAction, ActionBlock, ActionType, VariableDefinition};
use crate::shared::ast::blocks::function::Function;

// context: Actions before current CallAction (from domain begins)
pub fn check_function_parameter_types(action: CallAction, target_function: Function, defined_variables: Vec<VariableDefinition>) -> bool {
    if action.arguments.len() == target_function.parameters.len() {
        for (index, target_func_param) in target_function.parameters.iter().enumerate() {
            if target_func_param.type_name != action.arguments[index].output_type {
                return false;
            }
        }
    }

    return true;
}
