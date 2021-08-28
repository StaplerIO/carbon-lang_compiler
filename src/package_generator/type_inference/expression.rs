use crate::shared::ast::blocks::expression::{Expression, ExprTerm, TermType, ExprDataTerm, ExprDataTermType};
use crate::shared::ast::action::VariableDefinition;
use crate::shared::ast::blocks::function::Function;

// Term must be DataTerm
pub fn infer_expression_term_data_type(term: ExprDataTerm, defined_functions: Vec<Function>, defined_variables: Vec<VariableDefinition>) -> Option<String> {
    return match term.clone().data_type {
        ExprDataTermType::Number => Option::from(String::from("number")),
        ExprDataTermType::String => Option::from(String::from("str")),
        ExprDataTermType::Identifier => {
            for def_var in defined_variables {
                if def_var.identifier == term.clone().identifier.unwrap() {
                    return Option::from(def_var.identifier);
                }
            }

            None
        },
        ExprDataTermType::FunctionCall => {
            for def_func in defined_functions {
                if def_func.name == term.clone().function_call.unwrap().function_name {
                    return Option::from(def_func.return_type);
                }
            }

            None
        },
        _ => None
    }
}

pub fn infer_expression_output_type(expression: Expression, defined_types: Vec<String>) -> Option<String> {
    let mut possible_types = defined_types.clone();
    for term in expression.postfix_expr {
        let mut indexes_to_remove: Vec<usize> = vec![];
        // Find out impossible data type
        if term.term_type == TermType::Data {
            for current in possible_types.iter().enumerate() {
                if !is_castable_to_type(term.clone(), current.1.to_string()) {
                    indexes_to_remove.push(current.0);
                }
            }
        }

        // Delete indexes from the last one, prevent affecting the head of the possible_types array
        indexes_to_remove.reverse();

        // Remove impossible types
        for index in indexes_to_remove {
            possible_types.remove(index);
        }
    }

    return if possible_types.len() == 1 {
        Option::from(possible_types[0].clone())
    } else { None }
}

// TermType must be Data, check it before calling this function
fn is_castable_to_type(term: ExprTerm, target_type: String) -> bool {
    // Compiler defined types
    let data = term.data.unwrap();
    if data.type_name.unwrap() == target_type {
        return true;
    }

    // TODO: User defined types, try to find if the type can be casted into target type

    return false;
}
