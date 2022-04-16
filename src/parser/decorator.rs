use crate::shared::ast::decorated_token::{
    DataToken, DataType, DecoratedToken, DecoratedTokenType,
};
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::token::{Token, TokenContent};

pub fn decorate_token(tokens: Vec<Token>) -> Vec<DecoratedToken> {
    let mut result: Vec<DecoratedToken> = Vec::new();

    for token in tokens {
        match token.content {
            TokenContent::Identifier(x) => {
                // TODO: Check if this is a type name
                result.push(DecoratedToken {
                    token_type: DecoratedTokenType::Data,
                    data: Option::from(DataToken {
                        data_type: DataType::Identifier,
                        number: None,
                        string: None,
                        identifier: Option::from(x),
                        type_name: None,
                    }),
                    keyword: None,
                    container: None,
                    operator: None,
                });
            }
            TokenContent::Number(x) => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Data,
                data: Option::from(DataToken {
                    data_type: DataType::Number,
                    number: Option::from(x),
                    string: None,
                    identifier: None,
                    type_name: None,
                }),
                keyword: None,
                container: None,
                operator: None,
            }),
            TokenContent::String(x) => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Data,
                data: Option::from(DataToken {
                    data_type: DataType::String,
                    number: None,
                    string: Option::from(x),
                    identifier: None,
                    type_name: None,
                }),
                keyword: None,
                container: None,
                operator: None,
            }),
            TokenContent::Container(x) => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Container,
                data: None,
                keyword: None,
                container: Option::from(x),
                operator: None,
            }),
            TokenContent::Keyword(x) => match x {
                KeywordType::KwNumber => result.push(DecoratedToken {
                    token_type: DecoratedTokenType::Data,
                    data: Option::from(DataToken {
                        data_type: DataType::Type,
                        number: None,
                        string: None,
                        identifier: None,
                        type_name: Option::from(String::from("number")),
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
                    keyword: Option::from(x),
                    container: None,
                    operator: None,
                }),
            },
            TokenContent::Operator(x) => result.push(DecoratedToken {
                token_type: DecoratedTokenType::Operator,
                data: None,
                keyword: None,
                container: None,
                operator: Option::from(x),
            }),
            TokenContent::Semicolon => result.push(DecoratedToken {
                token_type: DecoratedTokenType::StatementEndSign,
                data: None,
                keyword: None,
                container: None,
                operator: None,
            }),
            _ => {}
        }
    }

    return result;
}
