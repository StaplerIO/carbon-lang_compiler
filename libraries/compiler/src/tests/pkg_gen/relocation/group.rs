use lazy_static::lazy_static;
use crate::lexer::tokenize::tokenize;
use crate::parser::decorator::decorate_token;
use crate::parser::pipeline::build_whole_file;
use crate::shared::utils::identifier::Identifier;
lazy_static! {
    static ref CONTENT: &'static str = r#"
        group Arc {
			field number foo(get, set);
			field str bar(get);

			method run()[none];
			method suspend()[number];

			func New()[Arc];
		}

		impl Arc {
			default foo = 2;
			default bar = "StaplerIO";

			field bar get {
				return bar;
			}

			method run()[none] { foo = foo + 1; }
			method suspend()[number] { return foo - 1; }

			func New()[Arc] { decl var number i_foo; }
		}
        "#;
}

#[test]
fn simple_group() {
    let tokens = decorate_token(tokenize(&CONTENT, true).unwrap()).0;
    let _structure = build_whole_file(tokens, Identifier::single("main"))
        .ok()
        .unwrap();
}
