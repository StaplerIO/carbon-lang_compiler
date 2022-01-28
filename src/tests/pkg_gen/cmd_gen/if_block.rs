mod tests {
    pub use crate::lexer::tokenize::tokenize;
    pub use crate::package_generator::command_builder::condition_command::if_command_builder;
    pub use crate::package_generator::command_builder::expression_evaluation::convert_number_to_hex;
    pub use crate::parser::builder::blocks::condition::if_block_builder;
    pub use crate::parser::decorator::decorate_token;
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

        assert_eq!(result.descriptors.len(), 4);

        // println!("{}", itertools::Itertools::join(&mut result.commands.iter(), ", "));
    }

    #[test]
    fn complex_if_test() {
        let tokens = tokenize("if (234 > 123) { decl var number foo; foo = 3; } elif (456 < 234) { decl var number bar; } else { }".to_string());

        let metadata = PackageMetadata {
            variable_slot_alignment: 2,
            data_alignment: 4,
            command_alignment: 0,
            entry_point_offset: 0,
            domain_layer_count_alignment: 2,
            address_alignment: 4
        };

        let result = if_command_builder(&if_block_builder(&decorate_token(tokens)).unwrap().0.if_action.unwrap(), &vec![], &metadata);

        assert_eq!(result.descriptors.len(), 8);
        // println!("{}", itertools::Itertools::join(&mut result.commands.iter(), ", "));
    }
}
