use crate::package_generator::type_inference::expression::infer_expression_term_data_type;
use crate::shared::ast::action::VariableDefinition;
use crate::shared::ast::blocks::expression::{Expression, TermType};
use crate::shared::ast::blocks::function::Function;

pub fn find_function(name: &str, available_functions: &Vec<Function>) -> Option<Function> {
    let result = available_functions.iter().find(|&e| e.name == name);
    if result.is_some() {
        return Option::from(result.unwrap().clone())
    }

    return None;
}

pub fn infer_every_expression_data_term_type(expression: &Expression, defined_functions: &Vec<Function>, defined_variables: &Vec<VariableDefinition>) -> Expression {
    let mut expr = expression.clone();

    for (index, term) in expr.postfix_expr.clone().iter().enumerate() {
        if term.term_type == TermType::Data {
            let mut data = term.data.clone().unwrap();
            data.type_name = infer_expression_term_data_type(&data, &defined_functions, &defined_variables);
            expr.postfix_expr[index].data = Option::from(data);
        }
    }

    return expr;
}

pub fn combine_command(master: u8, sub: u8) -> u8 {
    return master * 0x10 + sub;
}

/// Example:
/// `1234567` -> `[01, 23, 45, 67]`
/// `0x7c00` -> `[7c, 00]`
pub fn convert_to_u8_array(number: String) -> Vec<u8> {
    let mut result = vec![];

	for (index, digit) in number.chars().rev().enumerate() {
        // It will create an empty element first
        if index % 2 == 0 {
            result.insert(0, 0x00);
        }

        result[0] *= 0x10;
        if digit >= '0' && digit <= '9' {
            result[0] += (digit as u8 - '0' as u8) as u8;
        } else if digit >= 'a' && digit <= 'f' {
            result[0] += ((digit as u8 - 'a' as u8) + 0xA) as u8;
        } else if digit >= 'A' && digit <= 'F' {
            result[0] += ((digit as u8 - 'A' as u8) + 0xA) as u8;
        }
	}

    return result;
}

pub fn align_data_width(data_array: Vec<u8>, target_len: u8) -> Vec<u8> {
    let mut result = data_array.clone();

    // Align to width assigned in package_metadata
    if data_array.len() < target_len as usize {
        for _ in 0..(target_len as usize - data_array.len()) {
            result.insert(0, 0x00);
        }
    } else if data_array.len() > target_len as usize {
        panic!("Data width is too short, consider changing it into a longer width")
    }

    return result;
}

pub fn string_to_hex_char(s: String) -> char {
    return if s.eq("0"){'0'}
    else if s.eq("1"){'1'}
    else if s.eq("2"){'2'}
    else if s.eq("3"){'3'}
    else if s.eq("4"){'4'}
    else if s.eq("5"){'5'}
    else if s.eq("6"){'6'}
    else if s.eq("7"){'7'}
    else if s.eq("8"){'8'}
    else if s.eq("9"){'9'}
    else if s.eq("10"){'A'}
    else if s.eq("11"){'B'}
    else if s.eq("12"){'C'}
    else if s.eq("13"){'D'}
    else if s.eq("14"){'E'}
    else if s.eq("15"){'F'}
    else { '-' }
}
