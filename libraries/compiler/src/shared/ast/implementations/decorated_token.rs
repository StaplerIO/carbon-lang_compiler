use crate::shared::ast::decorated_token::{DataToken, DecoratedTokenContent};
use crate::shared::package_generation::data_descriptor::StringConstant;
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;
use crate::shared::utils::identifier::Identifier;

impl DecoratedTokenContent {
    pub fn eq_entry(&self, dtkc: &DecoratedTokenContent) -> bool {
        match (self, dtkc) {
            (DecoratedTokenContent::DecoratedKeyword(_), DecoratedTokenContent::DecoratedKeyword(_)) => true,
            (DecoratedTokenContent::Container(_), DecoratedTokenContent::Container(_)) => true,
            (DecoratedTokenContent::Data(_), DecoratedTokenContent::Data(_)) => true,
            (DecoratedTokenContent::Operator(_), DecoratedTokenContent::Operator(_)) => true,
            (DecoratedTokenContent::StatementEndSign, DecoratedTokenContent::StatementEndSign) => true,
            (_, _) => false
        }
    }

    pub fn get_decorated_keyword(&self) -> Option<&KeywordType> {
        match self {
            DecoratedTokenContent::DecoratedKeyword(x) => Option::from(x),
            _ => None
        }
    }

    pub fn get_operator(&self) -> Option<&Operator> {
        match self {
            DecoratedTokenContent::Operator(x) => Option::from(x),
            _ => None
        }
    }

    pub fn get_data(&self) -> Option<&DataToken> {
        match self {
            DecoratedTokenContent::Data(x) => Option::from(x),
            _ => None
        }
    }

    pub fn get_container(&self) -> Option<&ContainerType> {
        match self {
            DecoratedTokenContent::Container(x) => Option::from(x),
            _ => None
        }
    }

    pub fn get_statement_end_sign(&self) -> bool {
        return self == &DecoratedTokenContent::StatementEndSign;
    }

    pub fn is_valid_identifier(&self) -> bool {
        return match &self {
            DecoratedTokenContent::Data(x) => {
                match &x {
                    DataToken::Identifier(_) => true,
                    _ => false
                }
            }
            _ => false
        };
    }
}

impl DataToken {
    pub fn get_number(&self) -> Option<&String> {
        return match &self {
            DataToken::Number(x) => Option::from(x),
            _ => None
        };
    }

    pub fn get_identifier(&self) -> Option<&Identifier> {
        return match &self {
            DataToken::Identifier(x) => Option::from(x),
            _ => None
        };
    }

    pub fn get_string(&self) -> Option<&StringConstant> {
        return match &self {
            DataToken::String(x) => Option::from(x),
            _ => None
        };
    }
}
