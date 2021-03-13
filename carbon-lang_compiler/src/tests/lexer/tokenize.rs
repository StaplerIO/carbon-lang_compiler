mod tests {
    use crate::lexer::tokenize::tokenize;
    use crate::shared::token::{KeywordType, ContainerType, OperatorType};

    #[test]
    fn simple() {
        let result = tokenize(String::from("int 132211{ 32.85dd } >! = >="));

        assert_eq!(result.get(0).unwrap().identifier, String::from("int"));
        assert_eq!(result.get(1).unwrap().number, String::from("132211"));
        assert_eq!(result.get(3).unwrap().number, String::from("32.85"));
    }

    #[test]
    fn code_sample() {
        let result = tokenize(String::from("export func main() { std::io::println(\"123456\"); return 0; }"));

        assert_eq!(result.get(0).unwrap().keyword, KeywordType::KwExport);
        assert_eq!(result.get(1).unwrap().keyword, KeywordType::KwFunc);
        assert_eq!(result.get(2).unwrap().identifier, String::from("main"));
        assert_eq!(result.get(3).unwrap().container, ContainerType::Bracket);
        assert_eq!(result.get(4).unwrap().container, ContainerType::AntiBracket);
        assert_eq!(result.get(5).unwrap().container, ContainerType::Brace);
        assert_eq!(result.get(6).unwrap().identifier, String::from("std"));
        assert_eq!(result.get(7).unwrap().operator.operator_type, OperatorType::Scope);
        assert_eq!(result.get(8).unwrap().identifier, String::from("io"));
        assert_eq!(result.get(12).unwrap().string, String::from("123456"));
        assert_eq!(result.last().unwrap().container, ContainerType::AntiBrace);
    }
}
