use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::action_block::action_block_command_builder;
use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::action::ActionBlock;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

#[test]
fn action_block_no_condition() {
    let tokens = tokenize(
        "decl var number foo;\
                   decl var number bar;\
                   decl var number res;\
                   foo = 1;\
                   bar = 2;\
                   res = foo + bar;", true).unwrap();
    let actions = action_block_builder(
        decorate_token(tokens),
    ).unwrap();

    let metadata = PackageMetadata {
        package_type: 0,
        data_slot_alignment: 2,
        data_alignment: 8,
        global_command_offset: 0,
        domain_layer_count_alignment: 2,
        address_alignment: 4
    };

    let _commands = action_block_command_builder(&ActionBlock { actions }, false, &vec![], &metadata);

    // for element in commands {
    //     print!("{},", format!("{:X}", element));
    // }
}

#[test]
fn action_block_complete() {
    let tokens = tokenize(
        "decl var number foo;\
                   decl var number bar;\
                   foo = 0;\
                   bar = 2 + 4;\
                   if (foo > bar) {\
                       decl var number result;\
                       result = foo - bar;\
                   } else {\
                       decl var number res2;\
                       res2 = foo + bar;\
                   }"
    , true).unwrap();

    let actions = action_block_builder(
        decorate_token(tokens),
    ).unwrap();

    let metadata = PackageMetadata {
        data_slot_alignment: 2,
        data_alignment: 8,
        package_type: 0,
        global_command_offset: 5,
        domain_layer_count_alignment: 2,
        address_alignment: 8
    };

    let _commands = action_block_command_builder(&ActionBlock { actions }, false, &vec![], &metadata);
}
