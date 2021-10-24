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
