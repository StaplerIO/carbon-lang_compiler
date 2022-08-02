use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::package_generation::data_descriptor::{DataDeclarator, DataLocation};
use crate::shared::package_generation::package_descriptor::PackageMetadata;

#[test]
fn simple_test() {
    let tokens = tokenize("t = 1 + 2 * 3;", true).unwrap();
    let action = assignment_block_builder(&decorate_token(tokens).0)
        .ok()
        .unwrap()
        .0
        .get_assignment_action()
        .unwrap()
        .clone();

    let metadata = PackageMetadata {
        package_type: 0,
        data_slot_alignment: 2,
        data_alignment: 8,
        global_command_offset: 0,
        domain_layer_count_alignment: 2,
        address_alignment: 4
    };

    let defined_data = vec![DataDeclarator {
        name: "t".to_string(),
        slot: 0,
        location: DataLocation::Local,
        is_string: false
    }];

    let result = build_assignment_command(&action, &defined_data, &metadata);
    assert_eq!(
        result.commands,
        vec![177, 0, 0, 0, 0, 0, 0, 0, 0, 1, 177, 0, 0, 0, 0, 0, 0, 0, 0, 2, 177, 0, 0, 0, 0, 0, 0, 0, 0, 3, 241, 3, 241, 1, 180, 1, 0, 0]
    );

    // println!("{:?}", commands);
}
