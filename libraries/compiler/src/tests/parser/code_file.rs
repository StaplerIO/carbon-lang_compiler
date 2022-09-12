use lazy_static::lazy_static;

use crate::lexer::tokenize::tokenize;
use crate::parser::decorator::decorate_token;
use crate::parser::pipeline::build_whole_file;
use crate::shared::utils::identifier::Identifier;

lazy_static! {
    static ref FILE_CONTENT: &'static str = r#"
		link os;
		link std;

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
		
		decl func main(number foo)[number] {
			decl var number bar;
			bar = 2;
			decl var number result;
			result = arc::target(foo * bar);
			call f1();
			decl var str test;
			test = "Hello, world!";
			return result;
		}
		
		decl func arc::target(number v1)[number] {
			decl var number bar;
			bar = 2;
			decl var number result;
			result = v1 + bar;
			return result;
		}
		
		decl func f1()[none] {
			decl var number v2;
			v2 = 37413;
			if (v2 > 30000) {
				v2 = 1201;
			}
			
			return;
		}
        "#;
}

#[test]
fn file_example() {
    let tokens = decorate_token(tokenize(&FILE_CONTENT, true).unwrap()).0;
    let structure = build_whole_file(tokens, Identifier::single("main"))
        .ok()
        .unwrap();

    assert_eq!(structure.linked_code_files.len(), 2);
    assert_eq!(structure.functions.len(), 3);
}
