use crate::shared::ast::blocks::function::Function;

pub struct FunctionTable {
    pub entry: Function,
    // function_list doesn't contain the entry function
    pub function_list: Vec<Function>
}
