use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::ast::group::declaration::GroupDeclarationBlock;
use crate::shared::ast::group::implementation::{FieldImplementation, FunctionImplementation, GroupImplementationBlock, MethodImplementation};
use crate::shared::utils::identifier::Identifier;

impl GroupImplementationBlock {
    pub fn from_declaration(decl: &GroupDeclarationBlock) -> GroupImplementationBlock {
        let mut result = GroupImplementationBlock {
            source_group: decl.identifier.clone(),
            fields: vec![],
            methods: vec![],
            functions: vec![]
        };

        for field in &decl.fields {
            result.fields.push(FieldImplementation{
                identifier: field.identifier.clone(),
                get_block: None,
                set_block: None,
                default_value: SimpleExpression { postfix_expr: vec![], output_type: Identifier::empty() }
            });
        }

        for method in &decl.methods {
            result.methods.push(MethodImplementation{ declarator: method.clone(), body: vec![] });
        }

        for function in &decl.functions {
            result.functions.push(FunctionImplementation{declarator: function.clone(), body: vec![] });
        }

        return result;
    }
}
