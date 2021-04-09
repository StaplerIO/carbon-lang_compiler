mod tests {
    use crate::shared::token::{Token, Operator, TokenType, KeywordType, ContainerType, OperatorType, CalculationOperator, RelationOperator, LogicalOperator};

    #[test]
    fn expression_test() {
        let tokens: Vec<Token> = vec![Token {
            token_type: TokenType::Container,
            number: "".to_string(),
            string: "".to_string(),
            identifier: "".to_string(),
            keyword: KeywordType::Unset,
            container: ContainerType::Bracket,
            operator: Operator {
                operator_type: OperatorType::Unset,
                calculation: CalculationOperator::Unset,
                relation: RelationOperator::Unset,
                logical: LogicalOperator::Unset
            }
        },
        Token {
            token_type: TokenType::Container,
            number: "".to_string(),
            string: "".to_string(),
            identifier: "".to_string(),
            keyword: KeywordType::Unset,
            container: ContainerType::AntiBracket,
            operator: Operator {
                operator_type: OperatorType::Unset,
                calculation: CalculationOperator::Unset,
                relation: RelationOperator::Unset,
                logical: LogicalOperator::Unset
            }
        }];
    }
}