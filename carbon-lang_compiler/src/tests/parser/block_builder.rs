mod tests {
    use crate::shared::token::{Token, TokenType, ContainerType};

    #[test]
    fn expression_test() {
        let tokens: Vec<Token> = vec![Token {
            token_type: TokenType::Container,
            number: None,
            string: None,
            identifier: None,
            keyword: None,
            container: Option::from(ContainerType::Bracket),
            operator: None
        },
        Token {
            token_type: TokenType::Container,
            number: None,
            string: None,
            identifier: None,
            keyword: None,
            container: Option::from(ContainerType::AntiBracket),
            operator: None
        }];
    }
}