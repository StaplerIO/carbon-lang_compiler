mod tests {
    pub use crate::lexer::tokenize::tokenize;
    use crate::package_generator::command_builder::conditional_command::if_command_builder;
    use crate::parser::builder::blocks::condition::if_block_builder;
    use crate::parser::decorator::decorate_token;
    pub use crate::shared::package_generation::package_descriptor::PackageMetadata;

    #[test]
    fn bare_if_test() {
        let tokens = tokenize("if (234 > 123) { decl var number foo; foo = 3; }".to_string());

        let metadata = PackageMetadata {
            variable_slot_alignment: 2,
            data_alignment: 4,
            command_alignment: 0,
            entry_point_offset: 0,
            domain_layer_count_alignment: 2,
            address_alignment: 4
        };

        let result = if_command_builder(&if_block_builder(&decorate_token(tokens)).unwrap().0.if_action.unwrap(), &vec![], &metadata);

        assert_eq!(result.descriptors.len(), 3);
    }
}
