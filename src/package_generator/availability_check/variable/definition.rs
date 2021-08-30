use crate::shared::ast::action::{DeclarationAction, VariableDefinition};
use crate::package_generator::availability_check::variable::existence::check_variable_existence_by_name;

pub fn check_variable_definition(action: DeclarationAction, defined_variables: Vec<VariableDefinition>, defined_types: Vec<String>) -> bool {
    // If the type of variable is available
    if defined_types.contains(&action.data_type) {
        return !check_variable_existence_by_name(action.identifier, defined_variables);
    }

    return false;
}
