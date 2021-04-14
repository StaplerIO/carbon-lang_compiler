use crate::shared::token::RelationOperator;

// Reference at src/shared/token.rs
pub enum ExpressionOperatorType {
    Plus,
    Minus,
    Time,
    Divide,
    Mod,

    Equal,
    NotEqual,
    Bigger,
    BiggerEqual,
    Less,
    LessEqual,

    And,
    Or
}

pub struct Expression {
    pub is_single_term: bool,

    pub is_left_nested_expr: bool,
    pub is_right_nested_expr: bool,

    pub left_expr: Box<Option<Expression>>,
    pub right_expr: Box<Option<Expression>>,

    pub left_term: Option<Term>,
    pub right_term: Option<Term>,

    pub single_term: Option<Term>
}

pub enum TermType {
    Identifier,
    Number,
    String,
    Unset
}

pub struct Term {
    pub kind: TermType,
    pub identifier: Option<String>,
    pub number: Option<String>,
    pub string: Option<String>,

    // !foo -> True  |  foo -> False
    // Not available when Term is a String
    pub marked_as_not: Option<bool>
}
