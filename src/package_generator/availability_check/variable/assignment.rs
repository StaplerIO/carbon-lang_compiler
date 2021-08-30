use crate::package_generator::availability_check::expression::expr_sequence::check_expression_sequence;
use crate::package_generator::availability_check::variable::existence::check_variable_existence_by_name;
use crate::package_generator::type_inference::expression::infer_expression_output_type;
use crate::shared::ast::action::{AssignmentAction, VariableDefinition};

pub fn check_variable_assignment(action: AssignmentAction, defined_variables: Vec<VariableDefinition>, defined_types: Vec<String>) -> bool {
    if check_variable_existence_by_name(action.identifier, defined_variables) {
        if check_expression_sequence(action.eval_expression.clone()) {
            if infer_expression_output_type(action.eval_expression.clone(), defined_types).is_some() {
                return true;
            }
        }
    }

    return false;
}
