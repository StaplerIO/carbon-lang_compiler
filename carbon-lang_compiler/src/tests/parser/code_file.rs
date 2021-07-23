mod tests {
    pub use std::fs::read_to_string;
    pub use std::env::current_dir;
    pub use crate::parser::decorator::decorate_token;
    pub use crate::lexer::tokenize::tokenize;
    use crate::parser::pipeline::build_whole_file;

    #[test]
    fn file_example() {
        let mut sample_file_path = current_dir().unwrap();
        sample_file_path.extend(&["src", "tests", "sample.cbs"]);
        let content = read_to_string(sample_file_path.as_path()).unwrap();

        let tokens = decorate_token(tokenize(content));
        let structure = build_whole_file(tokens, String::from("main")).unwrap();

        assert_eq!(structure.linked_code_files.len(), 2);
        assert_eq!(structure.functions.len(), 2);
    }
}
