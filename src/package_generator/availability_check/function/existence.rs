use crate::shared::ast::blocks::function::Function;
use crate::shared::ast::action::CallAction;
use crate::package_generator::utils::find_function;

// Check CallAction
pub fn check_function_existance(function_table: Vec<Function>, action: CallAction) -> bool {
    return find_function(action.function_name, function_table).is_some();
}
