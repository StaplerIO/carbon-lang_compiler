mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::shared::token::KeywordType;

    #[test]
    fn simple() {
        let result = tokenize(String::from("int 132211{ 32.85dd }"));

        assert_eq!(result.get(0).unwrap().identifier, String::from("int"));
        assert_eq!(result.get(1).unwrap().number, String::from("132211"));
        assert_eq!(result.get(3).unwrap().number, String::from("32.85"));
    }
}
