use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROOT_COMMAND_OPCODE: HashMap<RootCommand, u8> = [
        (RootCommand::Object, 0xA),
        (RootCommand::Stack, 0xB),
        (RootCommand::Domain, 0xC),
        (RootCommand::Jump, 0xD),
        (RootCommand::Function, 0xE),
        (RootCommand::Math, 0xF),
    ].iter().cloned().collect();

    pub static ref OBJECT_COMMAND_OPCODE: HashMap<ObjectCommand, u8> = [
        (ObjectCommand::Create, 0x1),
        (ObjectCommand::Destory, 0x2),
    ].iter().cloned().collect();

    pub static ref STACK_COMMAND_OPCODE: HashMap<StackCommand, u8> = [
        (StackCommand::Push, 0x1),
        (StackCommand::PushFromObject, 0x2),
        (StackCommand::Pop, 0x3),
        (StackCommand::PopToObject, 0x4),
    ].iter().cloned().collect();

    pub static ref DOMAIN_COMMAND_OPCODE: HashMap<DomainCommand, u8> = [
        (DomainCommand::Create, 0x1),
        (DomainCommand::Destory, 0x2),
    ].iter().cloned().collect();

    pub static ref JUMP_COMMAND_OPCODE: HashMap<JumpCommand, u8> = [
        (JumpCommand::ToOffset, 0x1),
        (JumpCommand::ByStackTop, 0x2),
    ].iter().cloned().collect();

    pub static ref FUNCTION_COMMAND_OPCODE: HashMap<FunctionCommand, u8> = [
        (FunctionCommand::Enter, 0x1),
        (FunctionCommand::LeaveWithoutValue, 0x2),
        (FunctionCommand::LeaveWithValue, 0x3),
    ].iter().cloned().collect();

    pub static ref MATH_COMMAND_OPCODE: HashMap<MathCommand, u8> = [
        (MathCommand::Calculation, 0x1),
        (MathCommand::Logical, 0x2),
    ].iter().cloned().collect();

    pub static ref MATH_CALC_COMMAND_OPCODE: HashMap<MathCalcCommand, u8> = [
        (MathCalcCommand::Plus, 0x1),
        (MathCalcCommand::Minus, 0x2),
        (MathCalcCommand::Times, 0x3),
        (MathCalcCommand::Divide, 0x4),
        (MathCalcCommand::Mod, 0x5),
    ].iter().cloned().collect();

    pub static ref MATH_RELATION_OPCODE: HashMap<MathRelationCommand, u8> = [
        (MathRelationCommand::And, 0x1),
        (MathRelationCommand::Or, 0x2),
        (MathRelationCommand::Not, 0x3),
    ].iter().cloned().collect();
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum RootCommand {
    Object,
    Stack,
    Domain,
    Jump,
    Function,
    Math,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ObjectCommand {
    Create,
    Destory
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum StackCommand {
    Push,
    PushFromObject,
    Pop,
    PopToObject,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum DomainCommand {
    Create,
    Destory,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum JumpCommand {
    ToOffset,
    ByStackTop,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum FunctionCommand {
    Enter,
    LeaveWithoutValue,
    LeaveWithValue,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathCommand {
    Calculation,
    Logical
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathCalcCommand {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathRelationCommand {
    And,
    Or,
    Not,
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

impl MathRelationCommand {
    pub fn to_opcode(&self) -> u8 {
        return MATH_RELATION_OPCODE[self];
    }
}