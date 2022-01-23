mod tests {
    pub use crate::lexer::tokenize::tokenize;
    pub use crate::package_generator::command_builder::expression_evaluation::convert_number_to_hex;
    pub use crate::package_generator::command_builder::function_call::build_function_call_command;
    pub use crate::parser::builder::blocks::call::call_action_builder;
    pub use crate::parser::decorator::decorate_token;
    pub use crate::shared::package_generation::data_descriptor::DataDeclaration;
    pub use crate::shared::package_generation::package_descriptor::PackageMetadata;

    #[test]
    fn function_call() {
        let tokens = tokenize("call foo(23, bar);".to_string());
        let action = call_action_builder(&decorate_token(tokens))
            .unwrap()
            .0
            .call_action
            .unwrap();

        let metadata = PackageMetadata {
            variable_slot_alignment: 2,
            data_alignment: 4,
            command_alignment: 0,
            entry_point_offset: 0,
            domain_layer_count_alignment: 2,
            address_alignment: 0
        };

        let defined_data = vec![DataDeclaration {
            name: "bar".to_string(),
            slot: vec![0x00, 0x00],
            is_global: false,
        }];

        let _commands = build_function_call_command(&action, &defined_data, &metadata, &vec![]);

        // for element in commands {
        //     print!("{},", convert_number_to_hex(element.to_string()));
        // }
    }
}
