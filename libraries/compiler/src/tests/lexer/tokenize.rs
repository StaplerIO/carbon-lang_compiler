use crate::lexer::tokenize::tokenize;
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;

#[test]
fn simple() {
    let result = tokenize("number 132211{ 32.85 dd } >! = >= ,", false).unwrap();

    assert_eq!(
        result[0].get_keyword().unwrap(),
        KeywordType::KwNumber
    );
    assert_eq!(
        result[2].get_number().unwrap(),
        String::from("132211")
    );
    assert_eq!(
        result[5].get_number().unwrap(),
        String::from("32.85")
    );
    assert_eq!(
        result.last().unwrap().get_operator().unwrap(),
        Operator::Comma
    );
}

#[test]
fn code_sample() {
    let result = tokenize("export func main() { std::io::println(\"123456\"); return 0; }", false).unwrap();

    assert_eq!(
        result[0].get_keyword().unwrap(),
        KeywordType::KwExport
    );
    assert_eq!(
        result[2].get_keyword().unwrap(),
        KeywordType::KwFunc
    );
    assert_eq!(
        result[4].get_identifier().unwrap(),
        String::from("main")
    );
    assert_eq!(
        result[5].get_container().unwrap(),
        ContainerType::Bracket
    );
    assert_eq!(
        result[6].get_container().unwrap(),
        ContainerType::AntiBracket
    );
    assert_eq!(
        result[8].get_container().unwrap(),
        ContainerType::Brace
    );
    assert_eq!(
        result[10].get_identifier().unwrap(),
        String::from("std")
    );
    assert_eq!(
        result[11].get_operator().unwrap(),
        Operator::Scope
    );
    assert_eq!(
        result[12].get_identifier().unwrap(),
        String::from("io")
    );
    assert_eq!(
        result[16].get_string().unwrap(),
        String::from("123456")
    );
    assert_eq!(
        result.last().unwrap().get_container().unwrap(),
        ContainerType::AntiBrace
    );
}

#[test]
fn comment_test() {
    let result = tokenize("// comments here", false).unwrap();

    assert_eq!(result.len(), 1);
}
