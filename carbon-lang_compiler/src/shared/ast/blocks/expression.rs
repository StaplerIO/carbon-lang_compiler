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

    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,

    pub left_term: Box<Term>,
    pub right_term: Box<Term>,

    pub single_term: Box<Term>
}

pub enum TermType {
    Identifier,
    Number,
    String
}

pub struct Term {
    pub kind: TermType,
    pub identifier: String,
    pub number: String,
    pub string: String,

    // !foo -> True  |  foo -> False
    pub marked_as_not: bool
}
