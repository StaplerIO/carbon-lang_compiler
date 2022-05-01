use crate::shared::token::comment::Comment;
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;
use crate::shared::token::data::Number;
use crate::shared::utils::identifier::Identifier;
use crate::shared::utils::position::Position;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub content: TokenContent,
    pub position: Position,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenContent {
    Identifier(Identifier),
    Number(Number),
    String(String),
    Container(ContainerType),
    Keyword(KeywordType),
    Operator(Operator),
    Whitespace(String),
    Comment(Comment),
    Semicolon,
    Scope,
    Invalid,
}
