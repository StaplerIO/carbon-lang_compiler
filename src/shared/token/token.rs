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

impl Token {
    pub fn new(content: TokenContent, position: Position) -> Token {
        Token {
            content,
            position,
        }
    }

    pub fn new_invalid() -> Token {
        Token {
            content: TokenContent::Invalid,
            position: Position::empty(),
        }
    }

    pub fn is_invalid(&self) -> bool {
        self.content == TokenContent::Invalid
    }

    // Get value inside the enum TokenContent
    pub fn get_identifier(&self) -> Option<Identifier> {
        match self.content {
            TokenContent::Identifier(ref identifier) => Some(identifier.clone()),
            _ => None,
        }
    }

    pub fn get_number(&self) -> Option<Number> {
        match self.content {
            TokenContent::Number(ref number) => Some(number.clone()),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self.content {
            TokenContent::String(ref string) => Some(string.clone()),
            _ => None,
        }
    }

    pub fn get_container(&self) -> Option<ContainerType> {
        match self.content {
            TokenContent::Container(ref container) => Some(container.clone()),
            _ => None,
        }
    }

    pub fn get_keyword(&self) -> Option<KeywordType> {
        match self.content {
            TokenContent::Keyword(ref keyword) => Some(keyword.clone()),
            _ => None,
        }
    }

    pub fn get_operator(&self) -> Option<Operator> {
        match self.content {
            TokenContent::Operator(ref operator) => Some(operator.clone()),
            _ => None,
        }
    }

    pub fn get_whitespace(&self) -> Option<String> {
        match self.content {
            TokenContent::Whitespace(ref whitespace) => Some(whitespace.clone()),
            _ => None,
        }
    }
}
