use crate::shared::ast::blocks::function::Function;

pub struct AstTree {
    // function_list doesn't contain the entry function
    pub function_list: Vec<Function>,
    pub entry: Function,
}
