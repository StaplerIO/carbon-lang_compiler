use crate::package_generator::type_inference::expression::infer_expression_term_data_type;
use crate::shared::ast::action::{ActionBlock, ActionType, VariableDefinition};
use crate::shared::ast::blocks::expression::{Expression, TermType};
use crate::shared::ast::blocks::function::Function;

pub fn filter_action_by_action_type(action_type: ActionType, action_block: ActionBlock) -> ActionBlock {
    let mut result = ActionBlock {
        actions: vec![]
    };

    for action in action_block.actions {
        if action.action_type == action_type {
            result.actions.push(action.clone());
        }
    }

    return result;
}

pub fn find_function(name: String, available_functions: Vec<Function>) -> Option<Function> {
    for func in available_functions {
        if func.name == name {
            return Option::from(func);
        }
    }

    return None;
}

pub fn infer_every_expression_data_term_type(expression: Expression, defined_functions: Vec<Function>, defined_variables: Vec<VariableDefinition>) -> Expression {
    let mut expr = expression.clone();

    for (index, term) in expr.postfix_expr.clone().iter().enumerate() {
        if term.term_type == TermType::Data {
            let mut data = term.data.clone().unwrap();
            data.type_name = infer_expression_term_data_type(data.clone(), defined_functions.clone(), defined_variables.clone());
            expr.postfix_expr[index].data = Option::from(data);
        }
    }

    return expr;
}
