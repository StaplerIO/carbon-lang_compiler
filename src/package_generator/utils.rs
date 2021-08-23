use crate::shared::ast::action::{ActionType, ActionBlock};
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
