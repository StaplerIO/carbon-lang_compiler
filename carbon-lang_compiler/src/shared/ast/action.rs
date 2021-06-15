use crate::shared::ast::blocks::expression::Expression;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ActionType {
    DeclarationStatement,
    AssignmentStatement,
    CallStatement,
    ReturnStatement,
    IfStatement,
    WhileStatement,
    LoopStatement,
    SwitchStatement,
    BreakStatement,
    ContinueStatement,
    EmptyAction
}

// TODO: Builder required
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

    // "break" and "continue" actions don't have special blocks
}

pub struct ActionBlock {
    pub actions: Vec<Action>
}

// TODO: Builder required for if and elif statements
// Used in while, if, elif
pub struct ConditionBlock {
    pub condition: Expression,
    pub body: ActionBlock
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
    // Arguments are Expressions
    pub arguments: Vec<Expression>
}

pub struct ReturnAction {
    pub value: Expression
}

// TODO: Builder required
pub struct IfAction {
    pub if_block: ConditionBlock,

    pub elif_collection: Option<Vec<ConditionBlock>>,

    pub else_actions: Option<ActionBlock>
}

// TODO: Builder required
pub struct SwitchAction {
    pub condition: Expression,
    pub cases: Vec<SwitchCase>
}

// Builder with SwitchAction
pub struct SwitchCase {
    pub is_default: bool,
    pub value: String,
    pub actions: ActionBlock
}

// TODO: Builder required
pub struct LoopAction {
    pub actions: ActionBlock
}
