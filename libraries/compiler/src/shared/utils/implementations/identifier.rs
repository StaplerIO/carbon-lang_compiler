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

    pub fn build_identifier(tokens: &Vec<DecoratedToken>) -> Option<Identifier> {
        let mut result = Identifier::empty();
        let mut status: bool = false;
        for token in tokens.iter() {
            if status {
                if token.original_token.get_operator().unwrap_or(Operator::Invalid) == Operator::Scope {
                    break;
                }
                status = !status;
                continue;
            }

            let id = token.original_token.get_identifier();
            if id.is_some() {
                result.scope.push(id.unwrap());
                status = !status;
            } else {
                break;
            }
        }

        return if result.scope.is_empty() {
            None
        } else {
            result.name = result.scope.pop().unwrap();
            Some(result)
        };
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.scope.clone();
        v.push(self.name.clone());
        f.write_str(Itertools::join(&mut v.iter(), "::").as_str())
    }
}
