use crate::shared::ast::action::VariableDefinition;
use crate::shared::utils::identifier::Identifier;

pub fn check_variable_existence_by_name(
    identifier: &Identifier,
    defined_variables: &Vec<VariableDefinition>,
) -> bool {
    for var in defined_variables {
        if var.identifier == *identifier {
            return true;
        }
    }

    return false;
}
