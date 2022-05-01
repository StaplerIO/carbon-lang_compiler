use crate::shared::ast::blocks::expression::{SimpleExpression, RelationExpression};
use crate::shared::ast::parameter::Parameter;
use crate::shared::token::token::Token;

pub type VariableDefinition = Parameter;

#[derive(Clone, PartialEq, Debug)]
pub enum ActionContent {
    DeclarationStatement(DeclarationAction),
    AssignmentStatement(AssignmentAction),
    CallStatement(CallAction),
    ReturnStatement(ReturnAction),
    IfBlock(IfAction),
    WhileStatement(WhileBlock),
    LoopBlock(LoopBlock),
    SwitchBlock(SwitchAction),
    // "break" and "continue" actions don't have special blocks
    BreakStatement,
    ContinueStatement,
    EmptyAction,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Action {
    pub content: ActionContent,

    // Original token that can be used for error reporting
    pub tokens: Vec<Token>
}

pub type LoopBlock = ActionBlock;

#[derive(Clone, PartialEq, Debug)]
pub struct ActionBlock {
    pub actions: Vec<Action>,
}

pub type IfBlock = ConditionBlock;
pub type ElifBlock = ConditionBlock;
pub type WhileBlock = ConditionBlock;

// Used in while, if, elif
#[derive(Clone, PartialEq, Debug)]
pub struct ConditionBlock {
    pub condition: RelationExpression,
    pub body: ActionBlock,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DeclarationAction {
    // A variable or a constant
    pub is_variable: bool,

    pub identifier: String,
    pub data_type: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AssignmentAction {
    pub identifier: String,
    pub eval_expression: SimpleExpression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CallAction {
    pub function_name: String,
    // Arguments are Expressions
    pub arguments: Vec<SimpleExpression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReturnAction {
    pub value: SimpleExpression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IfAction {
    pub if_block: ConditionBlock,
    pub elif_collection: Vec<ElifBlock>,
    pub else_action: Option<ActionBlock>,
}

// TODO: Builder required
#[derive(Clone, PartialEq, Debug)]
pub struct SwitchAction {
    pub condition: SimpleExpression,
    pub cases: Vec<SwitchCase>,
}

// Builder with SwitchAction
#[derive(Clone, PartialEq, Debug)]
pub struct SwitchCase {
    pub is_default: bool,
    pub value: String,
    pub actions: ActionBlock,
}
