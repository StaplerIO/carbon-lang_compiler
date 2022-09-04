use crate::lexer::tokenize::tokenize;
use crate::parser::builder::group::declaration::group_declaration_builder;
use crate::parser::builder::group::implementation::group_implementation_builder;
use crate::parser::decorator::decorate_token;
use lazy_static::lazy_static;

lazy_static! {
    static ref DECLARATOR: &'static str = r#"group Arc {
                                                field number foo(get, set);
                                                field str bar(get);

                                                method run()[none];
                                                method suspend()[number];

                                                func New()[Arc];
                                              }"#;
    static ref IMPLEMENTER: &'static str = r#"impl Arc {
	                                                default foo = 2;
	                                                default bar = "StaplerIO";

	                                                field bar get {
	                                                    return bar;
	                                                }

	                                                method run()[none] { foo = foo + 1; }
	                                                method suspend()[number] { return foo - 1; }

	                                                func New()[Arc] { decl var number i_foo; }
                                                 }"#;
}

#[test]
fn declarator() {
    let tokens = decorate_token(tokenize(&DECLARATOR, true).unwrap()).0;

    let group_result = group_declaration_builder(&tokens).unwrap().0;

    assert_eq!(group_result.identifier.name, "Arc".to_string());
    assert_eq!(group_result.fields.len(), 2);
    assert_eq!(group_result.methods.len(), 2);
    assert_eq!(group_result.functions.len(), 1);
}

#[test]
fn implementer() {
    let tokens = decorate_token(tokenize(&IMPLEMENTER, true).unwrap()).0;
    let declarator_token = decorate_token(tokenize(&DECLARATOR, true).unwrap()).0;
    let declarator = group_declaration_builder(&declarator_token).unwrap().0;

    let group_result = group_implementation_builder(&tokens, &vec![declarator.clone()])
        .unwrap()
        .0;

    assert_eq!(group_result.source_group, declarator.identifier);

    assert_eq!(group_result.fields.len(), declarator.fields.len());

    assert_eq!(group_result.methods.len(), declarator.methods.len());
    assert_eq!(group_result.functions.len(), declarator.functions.len());
}
