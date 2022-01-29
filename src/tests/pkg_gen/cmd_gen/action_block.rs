use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::action::ActionBlock;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

#[test]
fn action_block() {
    let tokens = tokenize(String::from(
        "decl var number foo;\
                                                                    decl var number bar;\
                                                                    decl var number res;\
                                                                    foo = 1;\
                                                                    bar = 2;\
                                                                    res = foo + bar;",
    ));
    let actions = crate::parser::builder::blocks::action_block::action_block_builder(
        decorate_token(tokens),
    );

    let metadata = PackageMetadata {
        variable_slot_alignment: 0,
        data_alignment: 8,
        command_alignment: 0,
        entry_point_offset: 0,
        domain_layer_count_alignment: 0,
        address_alignment: 0
    };

    let _commands = action_block_builder(&ActionBlock { actions }, &metadata);

    // for element in commands {
    //     print!("{},", format!("{:X}", element));
    // }
}
