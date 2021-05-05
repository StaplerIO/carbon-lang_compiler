use crate::shared::token::{ContainerType, Operator, KeywordType};

pub enum DecoratedTokenType {
    DecoratedKeyword,
    Container,
    Data,
    StatementEndSign,
    Operator,
    Unset
}

pub struct DecoratedToken {
    pub token_type: DecoratedTokenType,

    pub data: Option<DataToken>,
    pub keyword: Option<KeywordType>,
    pub container: Option<ContainerType>,
    pub operator: Option<Operator>
}

pub enum DataType {
    Number,
    String,
    Identifier,
    Type
}

pub struct DataToken {
    pub data_type: DataType,
    pub number: Option<String>,
    pub string: Option<String>,
    pub identifier: Option<String>,
    pub type_name: Option<String>
}
