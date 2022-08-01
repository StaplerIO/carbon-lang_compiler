use crate::shared::package_generation::data_descriptor::StringConstant;
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;
use crate::shared::token::token::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum DecoratedTokenContent {
    DecoratedKeyword(KeywordType),
    Container(ContainerType),
    Data(DataToken),
    Operator(Operator),
    StatementEndSign,
}

#[derive(Clone, PartialEq)]
pub struct DecoratedToken {
    pub content: DecoratedTokenContent,
    pub original_token: Token
}

#[derive(Clone, PartialEq, Debug)]
pub enum DataToken {
    Number(String),
    String(StringConstant),
    Identifier(String),
    Typename(String),
}
