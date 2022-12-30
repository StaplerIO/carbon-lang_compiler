use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::token::operator::Operator;
use crate::shared::utils::identifier::Identifier;

impl Identifier {
    pub fn single(name: &str) -> Identifier {
        Identifier { name: name.to_string(), scope: vec![] }
    }

    pub fn empty() -> Identifier {
        Identifier { name: "".to_string(), scope: vec![] }
    }

    pub fn append(&mut self, value: &str) {
        self.scope.push(self.name.clone());
        self.name = value.to_string();
    }

    pub fn to_string(&self) -> String {
        let mut result = self.scope.clone();
        result.push(self.name.clone());
        Itertools::join(&mut result.iter(), "::").to_string()
    }

    pub fn from_tokens(tokens: &Vec<DecoratedToken>) -> Option<(Identifier, usize)> {
        let mut result = Identifier::empty();
        let mut status: bool = false;
        let mut index = 0;
        for token in tokens.iter() {
            if status {
                if token.original_token.get_operator().unwrap_or(Operator::Invalid) == Operator::Scope {
                    break;
                }
                status = !status;
                index += 1;
                continue;
            }

            let id = token.original_token.get_identifier();
            if id.is_some() {
                result.scope.push(id.unwrap());
                status = !status;
                index += 1;
            } else {
                break;
            }
        }

        return if result.scope.is_empty() {
            None
        } else {
            result.name = result.scope.pop().unwrap();
            Some((result, index))
        };
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_str(self.to_string().as_str());
    }
}
