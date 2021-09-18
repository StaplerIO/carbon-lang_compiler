use crate::shared::token::{CalculationOperator, LogicalOperator, Operator, OperatorType, RelationOperator, Token, TokenType, KeywordType, ContainerType};

impl Token {
    pub fn new_number(num: String) -> Token {
        return Token {
            token_type: TokenType::Number,
            number: Option::from(num),
            string: None,
            identifier: None,
            keyword: None,
            container: None,
            operator: None,
        };
    }

    pub fn new_string(str: String) -> Token {
        return Token {
            token_type: TokenType::String,
            number: None,
            string: Option::from(str),
            identifier: None,
            keyword: None,
            container: None,
            operator: None,
        };
    }
    pub fn new_identifier(identifier: String) -> Token {
        return Token {
            token_type: TokenType::Identifier,
            number: None,
            string: None,
            identifier: Option::from(identifier),
            keyword: None,
            container: None,
            operator: None,
        };
    }

    pub fn new_keyword(keyword: KeywordType) -> Token {
        return Token {
            token_type: TokenType::Keyword,
            number: None,
            string: None,
            identifier: None,
            keyword: Option::from(keyword),
            container: None,
            operator: None,
        };
    }

    pub fn new_container(container: ContainerType) -> Token {
        return Token {
            token_type: TokenType::Container,
            number: None,
            string: None,
            identifier: None,
            keyword: None,
            container: Option::from(container),
            operator: None,
        };
    }

    pub fn new_operator(operator: Operator) -> Token {
        return Token {
            token_type: TokenType::Operator,
            number: None,
            string: None,
            identifier: None,
            keyword: None,
            container: None,
            operator: Option::from(operator),
        };
    }

    pub fn new_semicolon() -> Token {
        return Token {
            token_type: TokenType::Semicolon,
            number: None,
            string: None,
            identifier: None,
            keyword: None,
            container: None,
            operator: None,
        };
    }
}

impl Operator {
    pub fn new_calculation(calc: CalculationOperator) -> Operator {
        return Operator {
            operator_type: OperatorType::Calculation,
            calculation: Option::from(calc),
            relation: None,
            logical: None,
        };
    }

    pub fn new_relation(rel: RelationOperator) -> Operator {
        return Operator {
            operator_type: OperatorType::Relation,
            calculation: None,
            relation: Option::from(rel),
            logical: None,
        };
    }

    // Hey LOGItech :)
    // G502 Hero feels awesome!!
    pub fn new_logical(logi: LogicalOperator) -> Operator {
        return Operator {
            operator_type: OperatorType::Logical,
            calculation: None,
            relation: None,
            logical: Option::from(logi),
        };
    }
}
