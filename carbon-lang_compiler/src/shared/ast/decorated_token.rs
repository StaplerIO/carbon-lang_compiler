use crate::shared::token::{ContainerType, Operator, KeywordType};

#[derive(Clone, PartialEq, Debug)]
pub enum DecoratedTokenType {
    DecoratedKeyword,
    Container,
    Data,
    StatementEndSign,
    Operator
}

#[derive(Clone, PartialEq)]
pub struct DecoratedToken {
    pub token_type: DecoratedTokenType,

    pub data: Option<DataToken>,
    pub keyword: Option<KeywordType>,
    pub container: Option<ContainerType>,
    pub operator: Option<Operator>,
}

#[derive(Clone, PartialEq)]
pub enum DataType {
    Number,
    String,
    Identifier,
    Type,
}

#[derive(Clone, PartialEq)]
pub struct DataToken {
    pub data_type: DataType,
    pub number: Option<String>,
    pub string: Option<String>,
    pub identifier: Option<String>,
    pub type_name: Option<String>,
}

impl DecoratedToken {
    pub fn is_valid_type(&self) -> bool {
        if self.token_type == DecoratedTokenType::Data {
            return self.clone().data.unwrap().data_type == DataType::Type;
        }

        return false;
    }

    pub fn is_valid_identifier(&self) -> bool {
        if self.token_type == DecoratedTokenType::Data {
            return self.clone().data.unwrap().data_type == DataType::Identifier;
        }

        return false;
    }
}
