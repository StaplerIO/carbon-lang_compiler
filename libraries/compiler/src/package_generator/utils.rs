use apa::apa::modulo::modulo;

use crate::package_generator::type_inference::expression::infer_expression_term_data_type;
use crate::shared::ast::action::VariableDefinition;
use crate::shared::ast::blocks::expression::{ExprDataTerm, SimpleExpression, TermContent};
use crate::shared::ast::blocks::function::Function;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{
    RelocationReference, RelocationReferenceType,
};
use crate::shared::utils::identifier::Identifier;

pub fn find_function(name: &Identifier, available_functions: &Vec<Function>) -> Option<Function> {
    let result = available_functions.iter().find(|&e| e.declarator.identifier == *name);
    if result.is_some() {
        return Option::from(result.unwrap().clone());
    }

    return None;
}

pub fn infer_every_expression_data_term_type(
    expression: &SimpleExpression,
    defined_functions: &Vec<Function>,
    defined_variables: &Vec<VariableDefinition>,
) -> SimpleExpression {
    let mut expr = expression.clone();

    for (index, term) in expr.postfix_expr.clone().iter().enumerate() {
        if term.content.get_data_term().is_some() {
            let mut data = term.content.get_data_term().unwrap().clone();
            data = ExprDataTerm::Identifier(infer_expression_term_data_type(&data, &defined_functions, &defined_variables).unwrap());
            expr.postfix_expr[index].content = TermContent::Data(data);
        }
    }

    return expr;
}

pub fn combine_command(master: u8, sub: u8) -> u8 {
    return master * 0x10 + sub;
}

pub fn convert_number_to_hex(mut number: String) -> String {
    // Means it is already a hex number
    if number.starts_with("0x") {
        return number[2..].to_string();
    }

    // Otherwise, it is a decimal. Convert it to hex
    let mut result = String::new();
    while !number.eq("0") {
        let modulo_result = modulo(number.clone(), String::from("16"));

        result.push(string_to_hex_char(modulo_result.1));
        number = modulo_result.0;
    }

    result = result.chars().rev().collect();

    return result;
}

/// ## Example:
///
/// `1234567` -> `[01, 23, 45, 67]`
///
/// `0x7c00` -> `[7c, 00]`
pub fn convert_to_u8_array(number: String) -> Vec<u8> {
    let t = number.parse::<isize>().unwrap();
    let result = t.to_be_bytes();

    return result.to_vec();
}

pub fn align_array_width(data_array: &Vec<u8>, target_len: u8) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    if data_array.len() < target_len as usize {
        let placeholder = vec![0 as u8; target_len as usize - data_array.len()];

        result.extend(placeholder);
        result.extend(data_array);
    } else if data_array.len() > target_len as usize {
        result = data_array.clone();
        while result.len() > target_len as usize {
            if result[0] == 0x00 {
                result.remove(0);
            } else {
                panic!("Data width is too short, consider changing it into a longer width (data/target : {}/{})", data_array.len(), target_len);
            }
        }
    } else {
        result = data_array.clone();
    }
    return result;
}

pub fn string_to_hex_char(s: String) -> char {
    return if s.eq("0") {
        '0'
    } else if s.eq("1") {
        '1'
    } else if s.eq("2") {
        '2'
    } else if s.eq("3") {
        '3'
    } else if s.eq("4") {
        '4'
    } else if s.eq("5") {
        '5'
    } else if s.eq("6") {
        '6'
    } else if s.eq("7") {
        '7'
    } else if s.eq("8") {
        '8'
    } else if s.eq("9") {
        '9'
    } else if s.eq("10") {
        'A'
    } else if s.eq("11") {
        'B'
    } else if s.eq("12") {
        'C'
    } else if s.eq("13") {
        'D'
    } else if s.eq("14") {
        'E'
    } else if s.eq("15") {
        'F'
    } else {
        '-'
    };
}

pub fn jump_command_address_placeholder(metadata: &PackageMetadata) -> Vec<u8> {
    return vec![0x00].repeat(metadata.address_alignment as usize);
}

pub fn is_domain_create_command(reloc_ref: &RelocationReference) -> bool {
    return match reloc_ref.ref_type {
        RelocationReferenceType::FunctionEntrance(_) => true,
        RelocationReferenceType::IfEntrance => true,
        RelocationReferenceType::ElifEntrance => true,
        RelocationReferenceType::ElseEntrance => true,
        RelocationReferenceType::WhileEntrance => true,
        RelocationReferenceType::LoopEntrance => true,
        _ => false,
    };
}

pub fn is_domain_destroy_command(reloc_ref: &RelocationReference) -> bool {
    return match reloc_ref.ref_type {
        RelocationReferenceType::EndFunction(_) => true,
        RelocationReferenceType::EndIf => true,
        RelocationReferenceType::EndElif => true,
        RelocationReferenceType::EndElse => true,
        RelocationReferenceType::EndWhile => true,
        RelocationReferenceType::EndLoop => true,
        _ => false,
    };
}

pub fn is_iteration_interrupt_command(reloc_ref: &RelocationReference) -> bool {
    return match reloc_ref.ref_type {
        RelocationReferenceType::EndWhile => true,
        RelocationReferenceType::EndLoop => true,
        _ => false,
    };
}

pub fn is_iteration_head_command(reloc_ref: &RelocationReference) -> bool {
    return match reloc_ref.ref_type {
        RelocationReferenceType::WhileEntrance => true,
        RelocationReferenceType::LoopEntrance => true,
        _ => false,
    };
}

pub fn is_function_end_command(reloc_ref: &RelocationReference) -> bool {
    return match reloc_ref.ref_type {
        RelocationReferenceType::EndFunction(_) => true,
        _ => false,
    }
}

pub fn is_function_begin_command(reloc_ref: &RelocationReference) -> bool {
    return match reloc_ref.ref_type {
        RelocationReferenceType::FunctionEntrance(_) => true,
        _ => false,
    }
}
