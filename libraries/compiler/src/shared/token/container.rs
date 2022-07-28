#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ContainerType {
    Brace,       // {
    AntiBrace,   // }
    Bracket,     // (
    AntiBracket, // )
    Index,       // [
    AntiIndex,   // ]
    Invalid,
}

impl ContainerType {
    pub fn is_bracket(&self) -> bool {
        return match self {
            ContainerType::Bracket => true,
            ContainerType::AntiBracket => true,
            _ => false
        }
    }

    pub fn is_brace(&self) -> bool {
        return match self {
            ContainerType::Brace => true,
            ContainerType::AntiBrace => true,
            _ => false
        }
    }

    pub fn is_index(&self) -> bool {
        return match self {
            ContainerType::Index => true,
            ContainerType::AntiIndex => true,
            _ => false
        }
    }
}
