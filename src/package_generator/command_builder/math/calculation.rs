use crate::package_generator::utils::combine_command;
use crate::shared::command_map::{MathCalcCommand, MathCommand, RootCommand};

pub fn plus_command() -> Vec<u8> {
    return vec![
        combine_command(
            RootCommand::Math.to_opcode(),
            MathCommand::Calculation.to_opcode(),
        ),
        MathCalcCommand::Plus.to_opcode(),
    ];
}

pub fn minus_command() -> Vec<u8> {
    return vec![
        combine_command(
            RootCommand::Math.to_opcode(),
            MathCommand::Calculation.to_opcode(),
        ),
        MathCalcCommand::Minus.to_opcode(),
    ];
}

pub fn multiplication_command() -> Vec<u8> {
    return vec![
        combine_command(
            RootCommand::Math.to_opcode(),
            MathCommand::Calculation.to_opcode(),
        ),
        MathCalcCommand::Times.to_opcode(),
    ];
}

pub fn divide_command() -> Vec<u8> {
    return vec![
        combine_command(
            RootCommand::Math.to_opcode(),
            MathCommand::Calculation.to_opcode(),
        ),
        MathCalcCommand::Divide.to_opcode(),
    ];
}

pub fn mod_command() -> Vec<u8> {
    return vec![
        combine_command(
            RootCommand::Math.to_opcode(),
            MathCommand::Calculation.to_opcode(),
        ),
        MathCalcCommand::Mod.to_opcode(),
    ];
}

pub fn inverse_command() -> Vec<u8> {
    return vec![
        combine_command(
            RootCommand::Math.to_opcode(),
            MathCommand::Calculation.to_opcode(),
        ),
        MathCalcCommand::Inverse.to_opcode(),
    ];
}
