use crate::package_generator::availability_check::variable::existence::check_variable_existence_by_name;
use crate::shared::ast::action::{DeclarationAction, VariableDefinition};
use crate::shared::utils::identifier::Identifier;

pub fn check_variable_definition(
    action: &DeclarationAction,
    defined_variables: &Vec<VariableDefinition>,
    defined_types: &Vec<Identifier>,
) -> bool {
    // If the type is defined
    if defined_types.contains(&action.declarator.data_type) {
        // Then check the existence
        return !check_variable_existence_by_name(&action.declarator.identifier, defined_variables);
    }

    return false;
}
