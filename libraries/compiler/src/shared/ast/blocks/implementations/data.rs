use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};

use crate::shared::ast::blocks::data::{ArrayElementAccessor, DataAccessor, DataType};
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::token::container::ContainerType;
use crate::shared::utils::identifier::Identifier;

impl DataAccessor {
    pub fn get_singleton(&self) -> Option<&Identifier> {
        return match self {
            DataAccessor::Singleton(x) => Some(x),
            DataAccessor::ArrayElement(_) => None
        };
    }

    pub fn get_array_element(&self) -> Option<&ArrayElementAccessor> {
        return match self {
            DataAccessor::Singleton(_) => None,
            DataAccessor::ArrayElement(x) => Some(x)
        };
    }

    pub fn get_identifier(&self) -> &Identifier {
        return match self {
            DataAccessor::Singleton(x) => x,
            DataAccessor::ArrayElement(x) => &x.identifier
        };
    }

    pub fn get_identifier_mut(&mut self) -> &mut Identifier {
        return match self {
            DataAccessor::Singleton(x) => x,
            DataAccessor::ArrayElement(x) => x.identifier.borrow_mut()
        };
    }
}

impl Display for DataAccessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = self.get_identifier().to_string();
        match &self {
            DataAccessor::ArrayElement(_) => {
                result = result + "[...]";
            }
            _ => {}
        }

        return f.write_str(result.as_str());
    }
}

impl DataType {
    pub fn from_tokens(tokens: &Vec<DecoratedToken>) -> Option<(DataType, usize)> {
        let identifier_result = Identifier::from_tokens(tokens);
        if identifier_result.is_some() {
            let identifier = identifier_result.unwrap();

            if tokens[identifier.1].original_token.get_container().unwrap_or(ContainerType::Invalid) == ContainerType::Index &&
                tokens[identifier.1 + 1].original_token.get_container().unwrap_or(ContainerType::Invalid) == ContainerType::AntiIndex {
                return Some((DataType {
                    data_type_id: identifier.0,
                    is_array: true,
                }, identifier.1));
            } else {
                return Some((DataType {
                    data_type_id: identifier.0,
                    is_array: false,
                }, identifier.1 + 2));
            }
        }

        return None;
    }

    pub fn new_none() -> DataType {
        return DataType {
            data_type_id: Identifier::empty(),
            is_array: false,
        };
    }
}
