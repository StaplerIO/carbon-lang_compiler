use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::expression_evaluation::build_expression_evaluation_command;
use crate::parser::builder::expression_builder::expression_infix_to_postfix;
use crate::parser::builder::expression_builder::expression_term_decorator;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

// TODO: Validate the possibility of the test
#[test]
fn expression_with_number_only() {
    let tokens = tokenize("1 + 2 * 3", true).unwrap();
    let expression = SimpleExpression {
        postfix_expr: expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens.clone()))),
        output_type: "number".to_string(),
    };

    let metadata = PackageMetadata {
        variable_slot_alignment: 0,
        data_alignment: 8,
        package_type: 0,
        global_command_offset: 0,
        domain_layer_count_alignment: 0,
        address_alignment: 0
    };

    // This is very abstract, needs to be validated
    let result = build_expression_evaluation_command(&expression, &vec![], &metadata);
    assert_eq!(
        result.commands,
        vec![
            0xb1, 0, 0, 0, 0, 0, 0, 0, 0x01, 0xb1, 0, 0, 0, 0, 0, 0, 0, 0x02, 0xb1, 0, 0, 0, 0,
            0, 0, 0, 0x03, 0xf1, 0x03, 0xf1, 0x01
        ]
    );

    // println!("{:?}", commands);
}

#[test]
fn expression_with_defined_data() {
    let tokens = tokenize("a + b * 2", true).unwrap();
    let expression = SimpleExpression {
        postfix_expr: expression_infix_to_postfix(expression_term_decorator(decorate_token(tokens.clone()))),
        output_type: "number".to_string(),
    };

    let metadata = PackageMetadata {
        variable_slot_alignment: 2,
        data_alignment: 8,
        package_type: 0,
        global_command_offset: 0,
        domain_layer_count_alignment: 0,
        address_alignment: 0
    };

    let defined_data = vec![
        DataDeclarator {
            name: "a".to_string(),
            slot: vec![0x00, 0x00],
            is_global: false,
        },
        DataDeclarator {
            name: "b".to_string(),
            slot: vec![0x00, 0x01],
            is_global: false,
        },
    ];

    let result = build_expression_evaluation_command(&expression, &defined_data, &metadata);
    assert_eq!(
        result.commands,
        vec![
            0xb2, 0, 0, 0, 0xb2, 0, 0, 1, 0xb1, 0, 0, 0, 0, 0, 0, 0, 0x02, 0xf1, 0x03, 0xf1,
            0x01
        ]
    );

    // println!("{:?}", commands);
}
