use crate::shared::ast::action::{Action, ActionBlock, ActionContent};
use crate::shared::ast::blocks::function::Function;
use crate::shared::package_generation::linear_action_tree::{LinearAction, LinearActionTree, LinearActionType};

pub fn from_function(function: &Function) -> LinearActionTree {
    let mut result = LinearActionTree {
        action_array: vec![]
    };

    result.action_array.push(LinearAction {
        lat_content: LinearActionType::FunctionEntrance(function.clone()),
        original_action: Action::new(ActionContent::EmptyAction, vec![]),
    });

    result.action_array.extend(action_block_adapter(&ActionBlock { actions: function.body.clone() }));

    result.action_array.push(LinearAction {
        lat_content: LinearActionType::EndFunction,
        original_action: Action::new(ActionContent::EmptyAction, vec![]),
    });

    return result;
}

fn action_block_adapter(block: &ActionBlock) -> Vec<LinearAction> {
    let mut result = vec![];

    for action in block.actions.clone() {
        match action.content {
            ActionContent::DeclarationStatement(_) => result.extend(declaration_action_adapter(&action)),
            ActionContent::AssignmentStatement(_) => result.extend(assignment_action_adapter(&action)),
            ActionContent::CallStatement(_) => result.extend(call_action_adapter(&action)),
            ActionContent::ReturnStatement(_) => result.extend(return_action_adapter(&action)),
            ActionContent::IfBlock(_) => result.extend(if_action_adapter(&action)),
            ActionContent::WhileStatement(_) => result.extend(while_block_adapter(&action)),
            ActionContent::LoopBlock(_) => result.extend(loop_block_adapter(&action)),
            ActionContent::SwitchBlock(_) => {
                panic!("Unsupported action!");
            }
            ActionContent::BreakStatement => result.extend(break_action_adapter(&action)),
            ActionContent::ContinueStatement => result.extend(continue_action_adapter(&action)),
            ActionContent::EmptyAction => {}
        }
    }

    return result;
}

fn declaration_action_adapter(action: &Action) -> Vec<LinearAction> {
    return vec![LinearAction { lat_content: LinearActionType::DeclarationAction(action.get_declaration_action().unwrap().clone()), original_action: action.clone() }];
}

fn call_action_adapter(action: &Action) -> Vec<LinearAction> {
    return vec![LinearAction { lat_content: LinearActionType::CallAction(action.get_call_action().unwrap().clone()), original_action: action.clone() }];
}

fn assignment_action_adapter(action: &Action) -> Vec<LinearAction> {
    return vec![LinearAction { lat_content: LinearActionType::AssignmentAction(action.get_assignment_action().unwrap().clone()), original_action: action.clone() }];
}

fn continue_action_adapter(action: &Action) -> Vec<LinearAction> {
    return vec![LinearAction { lat_content: LinearActionType::ContinueStatement, original_action: action.clone() }];
}

fn break_action_adapter(action: &Action) -> Vec<LinearAction> {
    return vec![LinearAction { lat_content: LinearActionType::BreakStatement, original_action: action.clone() }];
}

fn return_action_adapter(action: &Action) -> Vec<LinearAction> {
    return vec![LinearAction { lat_content: LinearActionType::ReturnAction(action.get_return_action().unwrap().clone()), original_action: action.clone() }];
}

fn if_action_adapter(action: &Action) -> Vec<LinearAction> {
    let if_action = action.get_if_action().unwrap();

    let mut result = vec![LinearAction { lat_content: LinearActionType::IfEntrance(if_action.if_block.clone()), original_action: Action::new(ActionContent::EmptyAction, vec![]) }];

    result.extend(action_block_adapter(&if_action.if_block.body));

    for elif_block in &if_action.elif_collection {
        result.push(LinearAction { lat_content: LinearActionType::ElseIfEntrance(elif_block.clone()), original_action: action.clone() });
        result.extend(action_block_adapter(&elif_block.body));
    }

    if if_action.else_action.is_some() {
        result.push(LinearAction { lat_content: LinearActionType::ElseEntrance, original_action: action.clone() });
        result.extend(action_block_adapter(&ActionBlock { actions: if_action.clone().else_action.unwrap().actions }));
    }

    result.push(LinearAction {
        lat_content: LinearActionType::EndIf,
        original_action: Action::new(ActionContent::EmptyAction, vec![]),
    });

    return result;
}

fn while_block_adapter(action: &Action) -> Vec<LinearAction> {
    let while_block = action.get_while_block().unwrap();

    let mut result = vec![LinearAction { lat_content: LinearActionType::WhileEntrance(while_block.clone()), original_action: Action::new(ActionContent::EmptyAction, vec![]) }];

    result.extend(action_block_adapter(&while_block.body));

    result.push(LinearAction {
        lat_content: LinearActionType::EndWhile,
        original_action: Action::new(ActionContent::EmptyAction, vec![]),
    });

    return result;
}

fn loop_block_adapter(action: &Action) -> Vec<LinearAction> {
    let loop_block = action.get_loop_block().unwrap();

    let mut result = vec![LinearAction { lat_content: LinearActionType::LoopEntrance, original_action: Action::new(ActionContent::EmptyAction, vec![]) }];

    result.extend(action_block_adapter(&loop_block));

    result.push(LinearAction {
        lat_content: LinearActionType::EndLoop,
        original_action: Action::new(ActionContent::EmptyAction, vec![]),
    });

    return result;
}
