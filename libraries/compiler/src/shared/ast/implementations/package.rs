use crate::shared::ast::package::ParserPackageStructure;
use crate::shared::package_generation::func_table::{FunctionTable, FunctionTableEntry};

impl ParserPackageStructure {
    pub fn export_function_table(&self) -> FunctionTable {
        let mut result = vec![];
        for x in &self.functions {
            result.push(FunctionTableEntry{
                slot: result.len(),
                name: x.declarator.identifier.clone(),
                relocated_entry_address: 0
            });
        }

        return result;
    }
}
