use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::condition_command::if_command_builder;
use crate::parser::builder::blocks::condition::if_block_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

#[test]
fn bare_if_test() {
    let tokens = tokenize("if (234 > 123) { decl var number foo; foo = 3; }", true).unwrap();

    let metadata = PackageMetadata {
        data_slot_alignment: 2,
        data_alignment: 4,
        package_type: 0,
        global_command_offset: 0,
        domain_layer_count_alignment: 2,
        address_alignment: 4
    };

    let result = if_command_builder(&if_block_builder(&decorate_token(tokens).0).unwrap().0.get_if_action().unwrap(),  &vec![], &metadata);

    assert_eq!(result.descriptors.targets.len(), 3);

    // println!("{}", itertools::Itertools::join(&mut result.commands.iter(), ", "));
}

#[test]
fn complex_if_test() {
    let tokens = tokenize("if (234 > 123) { decl var number foo; foo = 3; } elif (456 < 234) { decl var number bar; } else { }", true).unwrap();

    let metadata = PackageMetadata {
        data_slot_alignment: 2,
        data_alignment: 4,
        package_type: 0,
        global_command_offset: 0,
        domain_layer_count_alignment: 2,
        address_alignment: 4
    };

    let result = if_command_builder(&if_block_builder(&decorate_token(tokens).0).unwrap().0.get_if_action().unwrap(), &vec![], &metadata);

    assert_eq!(result.descriptors.targets.len(), 8);

    // println!("{}", itertools::Itertools::join(&mut result.commands.iter(), ", "));
}
