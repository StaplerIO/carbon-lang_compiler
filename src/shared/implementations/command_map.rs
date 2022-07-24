use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::shared::command_map::{
    DomainCommand, FunctionCommand, JumpCommand, MathCalcCommand, MathCommand, MathLogicalCommand,
    ObjectCommand, RootCommand, StackCommand,
};

lazy_static! {
    pub static ref ROOT_COMMAND_OPCODE: HashMap<RootCommand, u8> = [
        (RootCommand::Object, 0xA),
        (RootCommand::Stack, 0xB),
        (RootCommand::Domain, 0xC),
        (RootCommand::Jump, 0xD),
        (RootCommand::Function, 0xE),
        (RootCommand::Math, 0xF),
    ]
    .iter()
    .cloned()
    .collect();
    pub static ref OBJECT_COMMAND_OPCODE: HashMap<ObjectCommand, u8> =
        [(ObjectCommand::Create, 0x1), (ObjectCommand::Destroy, 0x2),]
            .iter()
            .cloned()
            .collect();
    pub static ref STACK_COMMAND_OPCODE: HashMap<StackCommand, u8> = [
        (StackCommand::Push, 0x1),
        (StackCommand::PushFromObject, 0x2),
        (StackCommand::Pop, 0x3),
        (StackCommand::PopToObject, 0x4),
    ]
    .iter()
    .cloned()
    .collect();
    pub static ref DOMAIN_COMMAND_OPCODE: HashMap<DomainCommand, u8> =
        [(DomainCommand::Create, 0x1), (DomainCommand::Destroy, 0x2),]
            .iter()
            .cloned()
            .collect();
    pub static ref JUMP_COMMAND_OPCODE: HashMap<JumpCommand, u8> =
        [(JumpCommand::ToAbsolute, 0x1), (JumpCommand::ByStackTop, 0x2),]
            .iter()
            .cloned()
            .collect();
    pub static ref FUNCTION_COMMAND_OPCODE: HashMap<FunctionCommand, u8> = [
        (FunctionCommand::Enter, 0x1),
        (FunctionCommand::LeaveWithoutValue, 0x2),
        (FunctionCommand::LeaveWithValue, 0x3),
    ]
    .iter()
    .cloned()
    .collect();
    pub static ref MATH_COMMAND_OPCODE: HashMap<MathCommand, u8> =
        [(MathCommand::Calculation, 0x1), (MathCommand::Logical, 0x2),]
            .iter()
            .cloned()
            .collect();
    pub static ref MATH_CALC_COMMAND_OPCODE: HashMap<MathCalcCommand, u8> = [
        (MathCalcCommand::Plus, 0x1),
        (MathCalcCommand::Minus, 0x2),
        (MathCalcCommand::Times, 0x3),
        (MathCalcCommand::Divide, 0x4),
        (MathCalcCommand::Mod, 0x5),
        (MathCalcCommand::Inverse, 0x6)
    ]
    .iter()
    .cloned()
    .collect();
    pub static ref MATH_LOGICAL_OPCODE: HashMap<MathLogicalCommand, u8> = [
        (MathLogicalCommand::And, 0x1),
        (MathLogicalCommand::Or, 0x2),
        (MathLogicalCommand::Not, 0x3),
    ]
    .iter()
    .cloned()
    .collect();
}

impl RootCommand {
    pub fn to_opcode(&self) -> u8 {
        return ROOT_COMMAND_OPCODE[self];
    }
}

impl ObjectCommand {
    pub fn to_opcode(&self) -> u8 {
        return OBJECT_COMMAND_OPCODE[self];
    }
}

impl StackCommand {
    pub fn to_opcode(&self) -> u8 {
        return STACK_COMMAND_OPCODE[self];
    }
}

impl DomainCommand {
    pub fn to_opcode(&self) -> u8 {
        return DOMAIN_COMMAND_OPCODE[self];
    }
}

impl JumpCommand {
    pub fn to_opcode(&self) -> u8 {
        return JUMP_COMMAND_OPCODE[self];
    }
}

impl FunctionCommand {
    pub fn to_opcode(&self) -> u8 {
        return FUNCTION_COMMAND_OPCODE[self];
    }
}

impl MathCommand {
    pub fn to_opcode(&self) -> u8 {
        return MATH_COMMAND_OPCODE[self];
    }
}

impl MathCalcCommand {
    pub fn to_opcode(&self) -> u8 {
        return MATH_CALC_COMMAND_OPCODE[self];
    }
}

impl MathLogicalCommand {
    pub fn to_opcode(&self) -> u8 {
        return MATH_LOGICAL_OPCODE[self];
    }
}
