#[derive(Copy, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Number,
    String,
    Container,
    Keyword,
    Operator,
    Semicolon,
    Unset
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KeywordType
{
    KwDecimal,          // decimal
    KwInt,              // int
    KwChar,             // char
    KwStr,              // str
    KwVar,              // var
    KwConst,            // const
    KwExport,           // export
    KwFunc,             // func
    KwIf,               // if
    KwElseIf,           // elif
    KwElse,             // else
    KwWhile,            // while
    KwLoop,             // loop
    KwSwitch,           // switch
    KwContinue,         // continue
    KwBreak,            // break
    KwReturn,           // return
    KwCall,             // call
    Unset
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ContainerType{
    Brace,              // {
    AntiBrace,          // }
    Bracket,            // (
    AntiBracket,        // )
    Index,              // [
    AntiIndex,          // ]
    Unset
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,

    pub number: String,
    pub string: String,
    pub identifier: String,
    pub keyword: KeywordType,
    pub container: ContainerType,
    pub operator: Operator
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OperatorType {
    // Root type
    Calculation,
    Relation,
    Logical,
    // Absolute type
    Assignment,         // =
    Scope,              // ::
    Unset
}

#[derive(Copy, Clone, PartialEq)]
pub enum CalculationOperator {
    Plus,               // +
    Minus,              // -
    Times,              // *
    Divide,             // /
    Mod,                // %
    Unset
}

#[derive(Copy, Clone, PartialEq)]
pub enum RelationOperator
{
    Bigger,             // >
    BiggerEqual,        // >=
    Less,               // <
    LessEqual,          // <=
    NotEqual,           // <>,
    Equal,              // ==
    Unset
}

#[derive(Copy, Clone, PartialEq)]
pub enum LogicalOperator {
    Not,                // !
    And,                // &&
    Or,                 // ||
    Unset
}

#[derive(Copy, Clone)]
pub struct Operator {
    pub operator_type: OperatorType,

    pub calculation: CalculationOperator,
    pub relation: RelationOperator,
    pub logical: LogicalOperator
}
