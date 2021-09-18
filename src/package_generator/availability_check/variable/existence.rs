use crate::shared::ast::action::VariableDefinition;

pub fn check_variable_existence_by_name(identifier: &str, defined_variables: &Vec<VariableDefinition>) -> bool {
    for var in defined_variables {
        if var.identifier == identifier.to_string() {
            return true;
        }
    }

    return false;
}
