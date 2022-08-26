use crate::shared::ast::blocks::function::Function;
use crate::shared::utils::identifier::Identifier;

pub fn check_function_return_type(func: &Function, defined_types: &Vec<Identifier>) -> bool {
    return defined_types.contains(&func.return_type);
}
