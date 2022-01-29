use lazy_static::lazy_static;

use crate::lexer::tokenize::tokenize;
use crate::parser::decorator::decorate_token;
use crate::parser::pipeline::build_whole_file;

lazy_static! {
        static ref FILE_CONTENT: &'static str = r#"
        link std;
        link os;

        decl func demo()[none]
        {
            call println("demo function!");
        }

        decl func main()[number]
        {
            call println("Hello, world!");
            call demo();

            return 0;
        }
        "#;
    }

#[test]
fn file_example() {
    let tokens = decorate_token(tokenize(FILE_CONTENT.parse().unwrap()));
    let structure = build_whole_file(tokens, String::from("main")).ok().unwrap();

    assert_eq!(structure.linked_code_files.len(), 2);
    assert_eq!(structure.functions.len(), 2);
}
