use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType, DataToken, DataType};
use crate::shared::token::{Token, TokenType, KeywordType};

pub fn decorate_token(tokens: Vec<Token>) -> Vec<DecoratedToken> {
    let mut result: Vec<DecoratedToken> = Vec::new();

    for token in tokens {
        match token.token_type {
            TokenType::Identifier => {
                // TODO: Check if this is a type name
                result.push(DecoratedToken {
                    token_type: DecoratedTokenType::Data,
                    data: Option::from(DataToken {
                        data_type: DataType::Identifier,
                        number: None,
                        string: None,
                        identifier: token.identifier.clone(),
                        type_name: None,
                    }),
                    keyword: None,
                    container: None,
                    operator: None,
                });
            }
            TokenType::Number => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Data,
                data: Option::from(DataToken {
                    data_type: DataType::Number,
                    number: token.number.clone(),
                    string: None,
                    identifier: None,
                    type_name: None,
                }),
                keyword: None,
                container: None,
                operator: None,
            }),
            TokenType::String => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Data,
                data: Option::from(DataToken {
                    data_type: DataType::String,
                    number: None,
                    string: token.string.clone(),
                    identifier: None,
                    type_name: None,
                }),
                keyword: None,
                container: None,
                operator: None,
            }),
            TokenType::Container => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Container,
                data: None,
                keyword: None,
                container: token.container.clone(),
                operator: None,
            }),
            TokenType::Keyword => match token.keyword.unwrap() {
                KeywordType::KwInt =>
                    result.push(DecoratedToken {
                        token_type: DecoratedTokenType::Data,
                        data: Option::from(DataToken {
                            data_type: DataType::Type,
                            number: None,
                            string: None,
                            identifier: None,
                            type_name: Option::from(String::from("int")),
                        }),
                        keyword: None,
                        container: None,
                        operator: None,
                    }),
                KeywordType::KwDecimal => result.push(DecoratedToken {
                    token_type: DecoratedTokenType::Data,
                    data: Option::from(DataToken {
                        data_type: DataType::Type,
                        number: None,
                        string: None,
                        identifier: None,
                        type_name: Option::from(String::from("decimal")),
                    }),
                    keyword: None,
                    container: None,
                    operator: None,
                }),
                KeywordType::KwChar => result.push(DecoratedToken {
                    token_type: DecoratedTokenType::Data,
                    data: Option::from(DataToken {
                        data_type: DataType::Type,
                        number: None,
                        string: None,
                        identifier: None,
                        type_name: Option::from(String::from("char")),
                    }),
                    keyword: None,
                    container: None,
                    operator: None,
                }),
                KeywordType::KwStr => result.push(DecoratedToken {
                    token_type: DecoratedTokenType::Data,
                    data: Option::from(DataToken {
                        data_type: DataType::Type,
                        number: None,
                        string: None,
                        identifier: None,
                        type_name: Option::from(String::from("str")),
                    }),
                    keyword: None,
                    container: None,
                    operator: None,
                }),
                _ => result.push(DecoratedToken {
                    token_type: DecoratedTokenType::DecoratedKeyword,
                    data: None,
                    keyword: token.keyword.clone(),
                    container: None,
                    operator: None,
                })
            }
            TokenType::Operator => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Operator,
                data: None,
                keyword: None,
                container: None,
                operator: token.operator.clone(),
            }),
            TokenType::Semicolon => {
                result.push(DecoratedToken {
                    token_type: DecoratedTokenType::StatementEndSign,
                    data: None,
                    keyword: None,
                    container: None,
                    operator: None,
                })
            }
        }
    }

    return result;
}