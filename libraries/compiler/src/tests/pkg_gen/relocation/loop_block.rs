// use std::io::Write;
use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::condition_command::while_command_builder;
use crate::parser::builder::blocks::loops::while_action_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

#[test]
fn simple_while_test() {
    let tokens = tokenize("while (234 > 123) { if (234 > 245) { break; } else { continue; } }", true).unwrap();

    let metadata = PackageMetadata {
        data_slot_alignment: 2,
        data_alignment: 4,
        package_type: 0,
        global_command_offset: 0,
        domain_layer_count_alignment: 2,
        address_alignment: 4,
    };

    let mut result = while_command_builder(&while_action_builder(&decorate_token(tokens).0).unwrap().0.get_while_block().unwrap(), &vec![], &metadata);

    result.calculate_ref_to_target();
    result.apply_relocation(metadata.address_alignment);

    // let mut file = std::fs::File::create("F:\\test.cbp").unwrap();
    // file.write_all(result.commands.as_slice()).unwrap();
    //
    // println!("{}", itertools::Itertools::join(&mut result.commands.iter(), ", "));
}
