use crate::shared::ast::action::{Action, ActionType, AssignmentAction, CallAction, DeclarationAction, IfAction, LoopBlock, ReturnAction, SwitchAction, WhileBlock};

impl Action {
    pub fn new_decl(decl: DeclarationAction) -> Action {
        return Action {
            action_type: ActionType::DeclarationStatement,
            declaration_action: Option::from(decl),
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: None
        }
    }

    pub fn new_assignment(assignment: AssignmentAction) -> Action {
        return Action {
            action_type: ActionType::AssignmentStatement,
            declaration_action: None,
            assignment_action: Option::from(assignment),
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: None
        }
    }

    pub fn new_call(call: CallAction) -> Action {
        return Action {
            action_type: ActionType::CallStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: Option::from(call),
            return_action: None,
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: None
        }
    }

    pub fn new_return(ret: ReturnAction) -> Action {
        return Action {
            action_type: ActionType::ReturnStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: Option::from(ret),
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: None
        }
    }

    pub fn new_if(if_action: IfAction) -> Action {
        return Action {
            action_type: ActionType::IfStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: Option::from(if_action),
            while_action: None,
            loop_action: None,
            switch_action: None
        }
    }

    pub fn new_while(while_action: WhileBlock) -> Action {
        return Action {
            action_type: ActionType::WhileStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: Option::from(while_action),
            loop_action: None,
            switch_action: None
        }
    }

    pub fn new_loop(loop_block: LoopBlock) -> Action {
        return Action {
            action_type: ActionType::LoopStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: None,
            loop_action: Option::from(loop_block),
            switch_action: None
        }
    }

    pub fn new_switch(switch: SwitchAction) -> Action {
        return Action {
            action_type: ActionType::SwitchStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: Option::from(switch)
        }
    }

    pub fn new_break() -> Action {
        return Action {
            action_type: ActionType::BreakStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: None
        };
    }

    pub fn new_continue() -> Action {
        return Action {
            action_type: ActionType::ContinueStatement,
            declaration_action: None,
            assignment_action: None,
            call_action: None,
            return_action: None,
            if_action: None,
            while_action: None,
            loop_action: None,
            switch_action: None
        };
    }
}
