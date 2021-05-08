#[derive(Copy, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Number,
    String,
    Container,
    Keyword,
    Operator,
    Semicolon,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KeywordType
{
    KwDecimal,          // decimal
    KwInt,              // int
    KwChar,             // char
    KwStr,              // str
    KwDeclare,          // decl
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
    KwNone,             // none
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

    pub number: Option<String>,
    pub string: Option<String>,
    pub identifier: Option<String>,
    pub keyword: Option<KeywordType>,
    pub container: Option<ContainerType>,
    pub operator: Option<Operator>
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CalculationOperator {
    Plus,               // +
    Minus,              // -
    Times,              // *
    Divide,             // /
    Mod,                // %
}

#[derive(Copy, Clone, PartialEq)]
pub enum RelationOperator
{
    Bigger,             // >
    BiggerEqual,        // >=
    Less,               // <
    LessEqual,          // <=
    NotEqual,           // <>
    Equal,              // ==
}

#[derive(Copy, Clone, PartialEq)]
pub enum LogicalOperator {
    Not,                // !
    And,                // &&
    Or,                 // ||
    Unset
}

#[derive(Copy, Clone, PartialEq)]
pub struct Operator {
    pub operator_type: OperatorType,

    pub calculation: Option<CalculationOperator>,
    pub relation: Option<RelationOperator>,
    pub logical: Option<LogicalOperator>
}
