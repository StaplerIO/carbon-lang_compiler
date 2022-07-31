use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::package_generation::data_descriptor::{DataDeclarator, DataLocation};
use crate::shared::package_generation::package_descriptor::PackageMetadata;

#[test]
fn simple_test() {
    let tokens = tokenize("t = 1 + 2 * 3;", true).unwrap();
    let action = assignment_block_builder(&decorate_token(tokens))
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
        vec![
            0xb1, 0, 0, 0, 0, 0, 0, 0, 0x01, 0xb1, 0, 0, 0, 0, 0, 0, 0, 0x02, 0xb1, 0, 0, 0, 0,
            0, 0, 0, 0x03, 0xf1, 0x03, 0xf1, 0x01, 0xb4, 1, 0, 0
        ]
    );

    // println!("{:?}", commands);
}
