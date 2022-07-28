use crate::shared::ast::action::{
    Action, ActionContent, AssignmentAction, CallAction, DeclarationAction, IfAction, LoopBlock,
    ReturnAction, SwitchAction, WhileBlock,
};
use crate::shared::token::token::Token;

impl Action {
    pub fn new(content: ActionContent, tokens: Vec<Token>) -> Action {
        Action {
            content,
            tokens,
        }
    }

    pub fn get_declaration_action(&self) -> Option<&DeclarationAction> {
        match &self.content {
            ActionContent::DeclarationStatement(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_assignment_action(&self) -> Option<&AssignmentAction> {
        match &self.content {
            ActionContent::AssignmentStatement(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_call_action(&self) -> Option<&CallAction> {
        match &self.content {
            ActionContent::CallStatement(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_if_action(&self) -> Option<&IfAction> {
        match &self.content {
            ActionContent::IfBlock(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_loop_block(&self) -> Option<&LoopBlock> {
        match &self.content {
            ActionContent::LoopBlock(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_return_action(&self) -> Option<&ReturnAction> {
        match &self.content {
            ActionContent::ReturnStatement(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_switch_action(&self) -> Option<&SwitchAction> {
        match &self.content {
            ActionContent::SwitchBlock(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_while_block(&self) -> Option<&WhileBlock> {
        match &self.content {
            ActionContent::WhileStatement(action) => Some(action),
            _ => None,
        }
    }

    pub fn get_content(&self) -> &ActionContent {
        &self.content
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
