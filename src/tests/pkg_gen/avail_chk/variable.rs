use crate::lexer::tokenize::tokenize;
use crate::package_generator::availability_check::variable::assignment::check_variable_assignment;
use crate::package_generator::availability_check::variable::definition::check_variable_definition;
use crate::package_generator::utils::infer_every_expression_data_term_type;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::builder::blocks::declaration::declaration_action_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::action::VariableDefinition;

#[test]
fn check_def() {
    let tokens = tokenize("decl var number a;");
    let stmt = declaration_action_builder(&decorate_token(tokens));

    let defined_types: Vec<String> = [
        String::from("number"),
        String::from("str"),
        String::from("char"),
    ]
        .to_vec();

    assert!(check_variable_definition(
        &stmt.ok().unwrap().0.declaration_action.unwrap(),
        &vec![],
        &defined_types
    ));
}

#[test]
fn check_assignment() {
    let tokens = tokenize("bcd = bcd + 2;");
    let stmt = assignment_block_builder(&decorate_token(tokens));

    let defined_vars: Vec<VariableDefinition> = [VariableDefinition {
        identifier: String::from("bcd"),
        type_name: String::from("number"),
    }]
        .to_vec();

    let defined_types: Vec<String> = [
        String::from("number"),
        String::from("str"),
        String::from("char"),
    ]
        .to_vec();

    let mut action = stmt.unwrap().0.assignment_action.unwrap();
    action.eval_expression =
        infer_every_expression_data_term_type(&action.eval_expression, &vec![], &defined_vars);

    assert!(check_variable_assignment(
        &action,
        defined_vars,
        defined_types
    ));
}
