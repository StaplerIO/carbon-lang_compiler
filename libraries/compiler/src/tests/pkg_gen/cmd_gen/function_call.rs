use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::parser::builder::blocks::call::call_action_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::package_generation::data_descriptor::{DataDeclarator, DataLocation};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::utils::identifier::Identifier;

#[test]
fn function_call() {
    let tokens = tokenize("call foo(23, bar);", true).unwrap();
    let action = call_action_builder(&decorate_token(tokens).0)
        .unwrap()
        .0
        .get_call_action()
        .unwrap()
        .clone();

    let metadata = PackageMetadata {
        data_slot_alignment: 2,
        data_alignment: 4,
        package_type: 0,
        global_command_offset: 0,
        domain_layer_count_alignment: 2,
        address_alignment: 0
    };

    let defined_data = vec![DataDeclarator {
        name: Identifier::single("bar"),
        slot: 0,
        location: DataLocation::Local,
        is_string: false
    }];

    let _commands = build_function_call_command(&action, &defined_data, &metadata);

    // for element in commands {
    //     print!("{},", convert_number_to_hex(element.to_string()));
    // }
}
