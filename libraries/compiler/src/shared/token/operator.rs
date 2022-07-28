#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Operator {
    // Root type
    Calculation(CalculationOperator),
    Relation(RelationOperator),
    Logical(LogicalOperator),
    // Absolute type
    Assignment, // =
    Scope,      // ::
    Comma,      // ,
    Invalid,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CalculationOperator {
    Addition,       // +
    Subtraction,    // -
    Multiply,       // *
    Division,       // /
    Modulo,         // %,
    Invalid
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RelationOperator {
    Greater,            // >
    GreaterOrEqual,     // >=
    Less,               // <
    LessOrEqual,        // <=
    NotEqual,           // <>
    Equal,              // ==,
    Invalid
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LogicalOperator {
    Not, // !
    And, // &&
    Or,  // ||
    Invalid,
}

impl Operator {
    pub fn eq_entry(&self, op: &Operator) -> bool {
        match (self, op) {
            (Operator::Calculation(_), Operator::Calculation(_)) => true,
            (Operator::Relation(_), Operator::Relation(_)) => true,
            (Operator::Logical(_), Operator::Logical(_)) => true,
            (Operator::Assignment, Operator::Assignment) => true,
            (Operator::Scope, Operator::Scope) => true,
            (Operator::Comma, Operator::Comma) => true,
            (Operator::Invalid, Operator::Invalid) => true,
            (_, _) => false }
    }

    pub fn get_calc_op(&self) -> Option<CalculationOperator> {
        match self {
            Operator::Calculation(op) => Some(*op),
            _ => None
        }
    }

    pub fn get_relation_op(&self) -> Option<RelationOperator> {
        match self {
            Operator::Relation(op) => Some(*op),
            _ => None
        }
    }

    pub fn get_logical_op(&self) -> Option<LogicalOperator> {
        match self {
            Operator::Logical(op) => Some(*op),
            _ => None
        }
    }
}
