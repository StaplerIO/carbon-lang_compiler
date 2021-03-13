pub enum TokenType {
    Identifier = 1,
    Number = 2,
    String = 3,
    Container = 4,
    Keyword = 5,
    Operator = 6,
    Unset = 0
}

pub enum KeywordType
{
    KwDecimal = 1,          // decimal
    KwInt = 2,              // int
    KwChar = 3,             // char
    KwStr = 4,              // str
    KwVar = 5,              // var
    KwConst = 6,            // const
    KwExport = 7,           // export
    KwFunc = 8,             // func
    KwIf = 9,               // if
    KwElseIf = 10,           // elif
    KwElse = 11,             // else
    KwWhile = 12,            // while
    KwLoop = 13,             // loop
    KwFor = 14,              // for
    KwSwitch = 15,           // switch
    KwContinue = 16,         // continue
    KwBreak = 17,            // break
    KwReturn = 18,           // return
    KwCall = 19,             // call
    Unset = 0
}

pub enum ContainerType{
    Brace = 1,              // {
    AntiBrace = 2,          // }
    Bracket = 3,            // (
    AntiBracket = 4,        // )
    Index = 5,              // [
    AntiIndex = 6,          // ]
    Unset = 0
}

pub struct Token {
    pub token_type: TokenType,

    pub number: String,
    pub string: String,
    pub identifier: String,
    pub keyword: KeywordType,
    pub container: ContainerType,
    pub operator: Operator
}

#[derive(Copy, Clone, PartialEq)]
pub enum OperatorType {
    // Root type
    Calculation = 1,
    Relation = 2,
    Logical = 3,
    // Absolute type
    Assignment = 4,         // =
    Scope = 5,              // ::
    Unset = 0
}

#[derive(Copy, Clone, PartialEq)]
pub enum CalculationOperator {
    Plus = 1,               // +
    Minus = 2,              // -
    Times = 3,              // *
    Divide = 4,             // /
    Mod = 5,                // %
    Unset = 0
}

#[derive(Copy, Clone, PartialEq)]
pub enum RelationOperator
{
    Bigger = 1,             // >
    BiggerEqual = 2,        // >=
    Less = 3,               // <
    LessEqual = 4,          // <=
    NotEqual = 5,           // <>,
    Equal = 6,              // ==
    Unset = 0
}

// Needs to be optimized
/*impl PartialEq for RelationOperator {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (RelationOperator::Bigger, RelationOperator::Bigger) => true,
            (RelationOperator::BiggerEqual, RelationOperator::BiggerEqual) => true,
            (RelationOperator::Less, RelationOperator::Less) => true,
            (RelationOperator::LessEqual, RelationOperator::LessEqual) => true,
            (RelationOperator::NotEqual, RelationOperator::NotEqual) => true,
            (RelationOperator::Equal, RelationOperator::Equal) => true,
            (RelationOperator::Unset, RelationOperator::Unset) => true,
            (_, _) => false
        }
    }
}*/

#[derive(Copy, Clone, PartialEq)]
pub enum LogicalOperator {
    Not = 1,                // !
    And = 2,                // &&
    Or = 3,                 // ||
    Unset = 0
}

#[derive(Copy, Clone)]
pub struct Operator {
    pub operator_type: OperatorType,

    pub calculation: CalculationOperator,
    pub relation: RelationOperator,
    pub logical: LogicalOperator
}
