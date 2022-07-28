use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
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
        variable_slot_alignment: 0,
        data_alignment: 8,
        command_alignment: 0,
        entry_point_offset: 0,
        domain_layer_count_alignment: 0,
        address_alignment: 0
    };

    let defined_data = vec![DataDeclarator {
        name: "t".to_string(),
        slot: vec![0x00, 0x00],
        is_global: false,
    }];

    let result = build_assignment_command(&action, &defined_data, &metadata);
    assert_eq!(
        result.commands,
        vec![
            0xb1, 0, 0, 0, 0, 0, 0, 0, 0x01, 0xb1, 0, 0, 0, 0, 0, 0, 0, 0x02, 0xb1, 0, 0, 0, 0,
            0, 0, 0, 0x03, 0xf1, 0x03, 0xf1, 0x01, 0xb4, 0, 0, 0
        ]
    );

    // println!("{:?}", commands);
}
