use crate::shared::ast::blocks::expression::{Expression, ExprTerm, TermType};
use crate::shared::ast::action::VariableDefinition;

pub fn infer_expression_output_type(expression: Expression, defined_types: Vec<String>, defined_variables: Vec<VariableDefinition>) -> Option<String> {
    let mut possible_types = defined_types.clone();
    for term in expression.postfix_expr {
        let mut indexes_to_remove: Vec<usize> = vec![];
        // Find out impossible data type
        if term.term_type == TermType::Data {
            for current in possible_types.iter().enumerate() {
                if !is_castable_to_type(term.clone(), current.1.to_string()) {
                    indexes_to_remove.insert(0, current.0);
                }
            }
        }

        // Remove impossible types
        for index in indexes_to_remove {
            possible_types.remove(index);
        }
    }

    return if possible_types.len() == 1 {
        Option::from(possible_types[0].clone())
    } else { None }
}

// TermType must be Data
fn is_castable_to_type(term: ExprTerm, target_type: String) -> bool {
    // Compiler defined types
    let data = term.data.unwrap();
    if data.type_name.unwrap() == target_type {
        return true;
    }

    // TODO: User defined types, try to find if the type can be casted to target type

    return false;
}
