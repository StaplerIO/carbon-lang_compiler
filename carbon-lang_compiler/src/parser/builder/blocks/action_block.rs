use crate::shared::ast::action::{Action, ActionType};
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::parser::builder::blocks::declaration::declare_data;
use crate::parser::builder::blocks::assignment::assignment_block;
use crate::parser::builder::blocks::call::call_function;
use crate::parser::builder::blocks::return_expression::build_return_statement;
use crate::parser::builder::blocks::short_actions::build_short_statements;

// Lookup lexer/tokenize.rs
pub fn action_block_builder(mut tokens: Vec<DecoratedToken>) -> Vec<Action> {
    let mut result: Vec<Action> = Vec::new();

    while tokens.len() > 0 {
        let decl = declare_data(tokens.clone());
        if decl.1 != -1 {
            result.push(Action{
                action_type: ActionType::DeclarationStatement,
                declaration_action: decl.0,
                assignment_action: None,
                call_action: None,
                return_action: None,
                if_action: None,
                while_action: None,
                loop_action: None,
                switch_action: None
            });

            tokens = tokens[((decl.1 as usize) + 1)..].to_vec();
            continue;
        }

        let mut assign_action = assignment_block(tokens.clone());
        if assign_action.1 != -1 {
            result.push(Action{
                action_type: ActionType::AssignmentStatement,
                declaration_action: None,
                assignment_action: assign_action.0,
                call_action: None,
                return_action: None,
                if_action: None,
                while_action: None,
                loop_action: None,
                switch_action: None
            });

            tokens = tokens[((assign_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        let mut call_action = call_function(tokens.clone());
        if call_action.1 > 0 {
            result.push(Action{
                action_type: ActionType::CallStatement,
                declaration_action: None,
                assignment_action: None,
                call_action: call_action.0,
                return_action: None,
                if_action: None,
                while_action: None,
                loop_action: None,
                switch_action: None
            });

            tokens = tokens[((call_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        let return_action = build_return_statement(tokens.clone());
        if return_action.1 > 0 {
            result.push(Action{
                action_type: ActionType::ReturnStatement,
                declaration_action: None,
                assignment_action: None,
                call_action: None,
                return_action: return_action.0,
                if_action: None,
                while_action: None,
                loop_action: None,
                switch_action: None
            });

            tokens = tokens[((return_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        let other_action = build_short_statements(tokens.clone());
        if other_action.1 != -1 {
            result.push(other_action.0.unwrap());

            tokens = tokens[((return_action.1 as usize) + 1)..].to_vec();
            continue;
        }

        panic!("Invalid token stream encountered");
    }

    return result;
}
