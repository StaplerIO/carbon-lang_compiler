use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref COMMAND_END_SIGN: u8 = 0x0;

    pub static ref RUNTIME_COMMAND_ROOT_OPCODE: HashMap<RuntimeCommandRootType, u8> = [
        (RuntimeCommandRootType::GlobalStack, 0xA),
        (RuntimeCommandRootType::Variable, 0xB),
        (RuntimeCommandRootType::EvaluateExpression, 0xC),
        (RuntimeCommandRootType::CompareValue, 0xD),
        (RuntimeCommandRootType::Jump, 0xE)
    ].iter().cloned().collect();

    pub static ref VARIABLE_COMMAND_OPCODE: HashMap<VariableCommand, u8> = [
        (VariableCommand::Create, 0x1),
        (VariableCommand::Clone, 0x2),
        (VariableCommand::Destroy, 0x3),
        (VariableCommand::SwitchFocus, 0x4),
        (VariableCommand::AssignValue, 0x5)
    ].iter().cloned().collect();

    pub static ref GLOBAL_STACK_COMMAND_OPCODE: HashMap<GlobalStackCommand, u8> = [
        (GlobalStackCommand::PushData, 0x1),
        (GlobalStackCommand::PopData, 0x2),
        (GlobalStackCommand::ViewTop, 0x3)
    ].iter().cloned().collect();

    pub static ref JUMP_COMMAND_OPCODE: HashMap<JumpCommand, u8> = [
        (JumpCommand::Offset, 0x1),
        (JumpCommand::ByStackTop, 0x2)
    ].iter().cloned().collect();
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum RuntimeCommandRootType {
    GlobalStack,
    Variable,
    // The result of the expression will be pushed to the top of `GlobalStack`
    EvaluateExpression,
    CompareValue,
    Jump,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum VariableCommand {
    Create,
    Clone,
    Destroy,
    SwitchFocus,
    // Value assigned is on the top of `GlobalStack`
    AssignValue,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum GlobalStackCommand {
    PushData,
    PopData,
    ViewTop,
}

/// ## About `ByStackTop`
///
/// It has 3 conditions: *zero*, *positive* and *negative*
///
/// The comparison object is the top data of the `GlobalStack`, after validated data, will execute an `JumpCommand.Offset` command, jump to target location
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum JumpCommand {
    Offset,
    ByStackTop,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MathRootCommand {
    Calculate,
    Logic,
    Relation,
}
