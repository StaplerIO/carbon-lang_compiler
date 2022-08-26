use crate::shared::ast::decorated_token::{
    DataToken, DecoratedToken, DecoratedTokenContent,
};
use crate::shared::package_generation::data_descriptor::StringConstant;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;
use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::identifier::Identifier;

pub fn decorate_token(tokens: Vec<Token>) -> (Vec<DecoratedToken>, Vec<StringConstant>) {
    let mut result: Vec<DecoratedToken> = Vec::new();

    let mut string_pool: Vec<StringConstant> = vec![];
    for token in tokens {
        match token.clone().content {
            TokenContent::Identifier(x) => {
                // TODO: Check if this is a type name
                result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Identifier(Identifier::single(x.as_str()))),
                    original_token: token.clone(),
                });
            }
            TokenContent::Number(x) => result.push(DecoratedToken {
                content: DecoratedTokenContent::Data(DataToken::Number(x)),
                original_token: token.clone(),
            }),
            TokenContent::String(x) => {
                if string_pool.iter().any(|s| s.value == x.clone()) {
                    result.push(DecoratedToken {
                        content: DecoratedTokenContent::Data(DataToken::String(string_pool.iter().find(|s| s.value == x.clone()).unwrap().clone())),
                        original_token: token.clone(),
                    });
                } else {
                    let constant = StringConstant { value: x.clone(), slot: string_pool.len() };
                    string_pool.push(constant.clone());
                    result.push(DecoratedToken {
                        content: DecoratedTokenContent::Data(DataToken::String(constant)),
                        original_token: token.clone(),
                    });
                }
            }
            TokenContent::Container(x) => result.push(DecoratedToken {
                content: DecoratedTokenContent::Container(x),
                original_token: token.clone(),
            }),
            TokenContent::Keyword(x) => match x {
                KeywordType::KwNumber => result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Typename(Identifier::single("number"))),
                    original_token: token.clone(),
                }),
                KeywordType::KwChar => result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Typename(Identifier::single("char"))),
                    original_token: token.clone(),
                }),
                KeywordType::KwStr => result.push(DecoratedToken {
                    content: DecoratedTokenContent::Data(DataToken::Typename(Identifier::single("str"))),
                    original_token: token.clone(),
                }),
                _ => result.push(DecoratedToken {
                    content: DecoratedTokenContent::DecoratedKeyword(x),
                    original_token: token.clone(),
                }),
            },
            TokenContent::Operator(x) => result.push(DecoratedToken {
                content: DecoratedTokenContent::Operator(x),
                original_token: token.clone(),
            }),
            TokenContent::Semicolon => result.push(DecoratedToken {
                content: DecoratedTokenContent::StatementEndSign,
                original_token: token.clone(),
            }),
            _ => {}
        }
    }

    result = post_combine_identifier(&result);

    return (result, string_pool);
}

#[inline]
fn post_combine_identifier(tokens: &Vec<DecoratedToken>) -> Vec<DecoratedToken> {
    let mut result: Vec<DecoratedToken> = vec![];
    let mut index = 0;
    while index < tokens.len() {
        if index <= tokens.len() - 2 {
            if tokens[index].original_token.content == TokenContent::Operator(Operator::Scope) &&
                tokens[index + 1].content.is_valid_identifier() &&
                result[result.len() - 1].content.is_valid_identifier() {
                // Update current identifier
                let result_len = result.len();
                let mut id = result[result_len - 1].content.get_data().unwrap().get_identifier().unwrap().clone();
                id.append(tokens[index + 1].content.get_data().unwrap().get_identifier().unwrap().name.as_str());
                result[result_len - 1].content = DecoratedTokenContent::Data(DataToken::Identifier(id));
                // Update index
                index += 2;
                continue;
            }
        }

        result.push(tokens[index].clone());
        index += 1;
    }

    return result;
}
