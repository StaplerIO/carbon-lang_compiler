use crate::shared::ast::decorated_token::{
    DataToken, DecoratedToken, DecoratedTokenContent,
};
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::token::{Token, TokenContent};

pub fn decorate_token(tokens: Vec<Token>) -> Vec<DecoratedToken> {
    let mut result: Vec<DecoratedToken> = Vec::new();

    for token in tokens {
        match token.clone().content {
            TokenContent::Identifier(x) => {
                // TODO: Check if this is a type name
                result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Identifier(x)),
                    original_token: token.clone()
                });
            }
            TokenContent::Number(x) => result.push(DecoratedToken {
                content: DecoratedTokenContent::Data(DataToken::Number(x)),
                original_token: token.clone()
            }),
            TokenContent::String(x) => result.push(DecoratedToken {
                content: DecoratedTokenContent::Data(DataToken::String(x)),
                original_token: token.clone()
            }),
            TokenContent::Container(x) => result.push(DecoratedToken {
                content: DecoratedTokenContent::Container(x),
                original_token: token.clone()
            }),
            TokenContent::Keyword(x) => match x {
                KeywordType::KwNumber => result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Typename("number".to_string())),
                    original_token: token.clone()
                }),
                KeywordType::KwChar => result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Typename("char".to_string())),
                    original_token: token.clone()
                }),
                KeywordType::KwStr => result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Typename("str".to_string())),
                    original_token: token.clone()
                }),
                _ => result.push(DecoratedToken {
                    content: DecoratedTokenContent::DecoratedKeyword(x),
                    original_token: token.clone()
                }),
            },
            TokenContent::Operator(x) => result.push(DecoratedToken {
                content: DecoratedTokenContent::Operator(x),
                original_token: token.clone()
            }),
            TokenContent::Semicolon => result.push(DecoratedToken {
                content: DecoratedTokenContent::StatementEndSign,
                original_token: token.clone()
            }),
            _ => {}
        }
    }

    return result;
}
