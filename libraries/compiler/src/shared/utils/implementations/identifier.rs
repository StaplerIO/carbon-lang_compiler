use std::fmt::{Display, Formatter};
use itertools::Itertools;
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
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.scope.clone();
        v.push(self.name.clone());
        f.write_str(Itertools::join(&mut v.iter(), "::").as_str())
    }
}
