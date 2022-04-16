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
