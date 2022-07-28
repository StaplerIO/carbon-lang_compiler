use crate::shared::ast::action::{Action,
                                 AssignmentAction,
                                 CallAction,
                                 ConditionBlock,
                                 DeclarationAction,
                                 ReturnAction,
                                 WhileBlock};
use crate::shared::ast::blocks::function::Function;

pub enum LinearActionType {
    FunctionEntrance(Function),
    EndFunction,
    IfEntrance(ConditionBlock),
    ElseIfEntrance(ConditionBlock),
    ElseEntrance,
    EndIf,
    WhileEntrance(WhileBlock),
    EndWhile,
    LoopEntrance,
    EndLoop,
    BreakStatement,
    ContinueStatement,
    AssignmentAction(AssignmentAction),
    DeclarationAction(DeclarationAction),
    CallAction(CallAction),
    ReturnAction(ReturnAction),
}

pub struct LinearAction {
    pub lat_content: LinearActionType,

    pub original_action: Action,
}

pub struct LinearActionTree {
    pub action_array: Vec<LinearAction>,
}

