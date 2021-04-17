use crate::shared::ast::blocks::expression::Expression;
use crate::shared::ast::parameter::Argument;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ActionType {
    Declaration,
    Assignment,
    CallStatement,
    ReturnStatement,
    IfStatement,
    WhileStatement,
    LoopStatement,
    SwitchStatement,
    BreakStatement,
    ContinueStatement,
}

pub struct Action {
    pub action_type: ActionType,

    pub declaration_action: Option<DeclarationAction>,
    pub assignment_action: Option<AssignmentAction>,

    pub call_action: Option<CallAction>,
    pub return_action: Option<ReturnAction>,
    pub if_action: Option<IfAction>,
    pub while_action: Option<ConditionBlock>,
    pub loop_action: Option<LoopAction>,
    pub switch_action: Option<SwitchAction>,

    // break and continue action don't have special blocks
}

pub struct ActionBlock {
    pub actions: Vec<Action>
}

// Used in while, if, elif
pub struct ConditionBlock {
    pub condition: Expression,
    pub body: ActionBlock,
}

pub struct DeclarationAction {
    // A variable or a constant
    pub is_variable: bool,

    pub identifier: String,
    pub data_type: String
}

pub struct AssignmentAction {
    pub identifier: String,
    pub eval_expression: Expression
}

pub struct CallAction {
    pub function_name: String,
    pub arguments: Vec<Argument>
}

pub struct ReturnAction {
    pub value: Expression
}

pub struct IfAction {
    pub if_block: ConditionBlock,

    pub elif_collection: Option<Vec<ConditionBlock>>,

    pub else_actions: Option<ActionBlock>
}

pub struct SwitchAction {
    pub condition: Expression,
    pub cases: Vec<SwitchCase>
}

pub struct SwitchCase {
    pub is_default: bool,
    pub value: String,
    pub actions: ActionBlock
}

pub struct LoopAction {
    pub actions: ActionBlock
}
