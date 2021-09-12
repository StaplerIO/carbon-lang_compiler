use crate::shared::token::{Operator, CalculationOperator, OperatorType, RelationOperator, LogicalOperator};

impl Operator {
    pub fn new_calculation(calc: CalculationOperator) -> Operator {
        return Operator {
            operator_type: OperatorType::Calculation,
            calculation: Option::from(calc),
            relation: None,
            logical: None
        };
    }

    pub fn new_relation(rel: RelationOperator) -> Operator {
        return Operator {
            operator_type: OperatorType::Relation,
            calculation: None,
            relation: Option::from(rel),
            logical: None
        };
    }

    // Hey LOGItech :)
    // G502 Hero feels awesome!!
    pub fn new_logical(logi: LogicalOperator) -> Operator {
        return Operator {
            operator_type: OperatorType::Logical,
            calculation: None,
            relation: None,
            logical: Option::from(logi)
        };
    }
}
