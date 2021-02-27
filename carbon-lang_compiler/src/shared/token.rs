enum TokenType {
    Identifier,
    Number,
    String,
    Container,
    Keyword,
}

enum KeywordType
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
    KwFor,              // for
    KwSwitch,           // switch
    KwContinue,         // continue
    KwBreak,            // break
    KwReturn,           // return
    KwCall,             // call
}

enum ContainerType {
    Brace,              // {
    AntiBrace,          // }
    Bracket,            // (
    AntiBracket,        // )
    Index,              // [
    AntiIndex           // ]
}

struct Token {
    pub token_type: TokenType,

    pub number: String,
    pub string: String,
    pub keyword: KeywordType,
    pub container: ContainerType
}

enum OperatorType {
    // Root type
    Calculation,
    Relation,
    Logical,
    // Absolute type
    Assignment,         // =
    Scope               // ::
}

enum CalculationOperator {
    Plus,               // +
    Minus,              // -
    Times,              // *
    Divide,             // /
    Mod                 // %
}

enum RelationOperator
{
    Bigger,             // >
    BiggerEqual,        // >=
    Less,               // <
    LessEqual,          // <=
    NotEqual            // <>
}

enum LogicalOperator {
    Not,                // !
    And,                // &&
    Or                  // ||
}

struct Operator {
    pub operator_type: OperatorType,

    pub calculation: CalculationOperator,
    pub relation: RelationOperator,
    pub logical: LogicalOperator
}
