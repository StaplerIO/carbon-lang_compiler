// use std::io::Write;

use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::function_block::build_function_command;
use crate::parser::decorator::decorate_token;
use crate::parser::pipeline::build_whole_file;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

#[test]
fn single_function() {
    let tokens = tokenize("decl func main(number foo)[number] { \
                                                    decl var number bar; \
                                                    bar = 2; \
                                                    decl var number result; \
                                                    result = foo + bar;\
                                                    return result;\
                                                }", true);
    let tree = build_whole_file(decorate_token(tokens), "main".to_string()).unwrap();
    let metadata = PackageMetadata {
        variable_slot_alignment: 2,
        data_alignment: 4,
        command_alignment: 2,
        entry_point_offset: 5,
        domain_layer_count_alignment: 2,
        address_alignment: 4,
    };

    let mut target = build_function_command(&tree.functions[0], &vec![], &metadata);

    // Write file
    // let mut file = std::fs::File::create("F:\\test.cbp").unwrap();

    let bytes = metadata.serialize();
    target.calculate_ref_to_target(bytes.len());
    target.apply_relocation(metadata.address_alignment);

    // bytes.extend(target.commands.clone());
    // file.write_all(bytes.as_slice()).unwrap();

    // println!("{}", itertools::Itertools::join(&mut target.commands.iter(), ", "));
}

#[test]
fn multiple_functions() {
    let tokens = tokenize("decl func main(number foo)[number] { \
                                                    decl var number bar; \
                                                    bar = 2; \
                                                    decl var number result; \
                                                    result = target(foo * bar);\
                                                    call f1();\
                                                    return result;\
                                                }\
                                                \
                                                decl func target(number v1)[number] { \
                                                    decl var number bar; \
                                                    bar = 2; \
                                                    decl var number result; \
                                                    result = v1 + bar;\
                                                    return result;\
                                                }\
                                                \
                                                decl func f1()[none] {\
                                                    decl var number v2;\
                                                    v2 = 37413;\
                                                }", true);
    let tree = build_whole_file(decorate_token(tokens), "main".to_string()).unwrap();
    let metadata = PackageMetadata {
        variable_slot_alignment: 2,
        data_alignment: 4,
        command_alignment: 2,
        entry_point_offset: 5,
        domain_layer_count_alignment: 2,
        address_alignment: 4,
    };

    let mut target = RelocatableCommandList::new();
    for func in &tree.functions {
        target.combine(build_function_command(func, &vec![], &metadata));
    }

    // Write file
    // let mut file = std::fs::File::create("F:\\test.cbp").unwrap();

    let bytes = metadata.serialize();
    target.calculate_ref_to_target(bytes.len());
    // target.apply_relocation(metadata.address_alignment);

    // bytes.extend(target.commands.clone());
    // file.write_all(bytes.as_slice()).unwrap();

    // println!("{}", itertools::Itertools::join(&mut target.commands.iter(), ", "));
}
