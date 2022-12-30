use crate::package_generator::availability_check::expression::expr_sequence::check_expression_sequence;
use crate::package_generator::availability_check::variable::existence::check_variable_existence_by_name;
use crate::package_generator::type_inference::expression::infer_expression_output_type;
use crate::shared::ast::action::{AssignmentAction, VariableDefinition};
use crate::shared::utils::identifier::Identifier;

pub fn check_variable_assignment(
    action: &AssignmentAction,
    defined_variables: Vec<VariableDefinition>,
    defined_types: Vec<Identifier>,
) -> bool {
    if check_variable_existence_by_name(action.lhs_accessor.get_identifier(), &defined_variables) {
        if check_expression_sequence(action.rhs_eval_expression.clone()) {
            if infer_expression_output_type(&action.rhs_eval_expression, &defined_types).is_some() {
                return true;
            }
        }
    }

    return false;
}
