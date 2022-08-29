use lazy_static::lazy_static;
use crate::lexer::tokenize::tokenize;
use crate::parser::builder::group::declaration::group_declaration_builder;
use crate::parser::decorator::decorate_token;

lazy_static! {
    static ref DECLARATOR: &'static str = r#"group Arc {
                                                field number foo(get, set);
                                                field str bar(get);

                                                method run()[none];
                                                method suspend()[number];

                                                func New()[Arc];
                                              }"#;
}

#[test]
fn declarator() {
    let tokens = decorate_token(tokenize(&DECLARATOR, true).unwrap()).0;

    let group_result = group_declaration_builder(&tokens);

	assert!(group_result.is_ok());
}
