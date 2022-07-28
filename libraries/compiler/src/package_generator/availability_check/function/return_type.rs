use crate::shared::ast::blocks::function::Function;

pub fn check_function_return_type(func: &Function, defined_types: &Vec<String>) -> bool {
    return defined_types.contains(&func.return_type);
}
