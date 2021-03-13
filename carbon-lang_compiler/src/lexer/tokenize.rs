/**
 * Regular expression sequence
 * Number: `\d+(\.\d+)?`
 * String: `"[^"]*"`
 * Identifier: `[a-zA-Z_]([a-zA-Z_0-9])*`
 */

use crate::shared::token::*;
use crate::lexer::hard_code::lex_rule::*;
use crate::lexer::hard_code::match_enums::match_keyword;

pub fn tokenize(mut source_code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    while source_code.len() > 0 {
        #[allow(unused_assignments)]
            let mut lexeme = String::new();

        lexeme = match_identifier(source_code.clone());
        if lexeme.len() > 0 {
            let token = Token {
                token_type: TokenType::Identifier,
                number: "".to_string(),
                string: "".to_string(),
                identifier: lexeme.clone(),
                keyword: match_keyword(lexeme.clone()),
                container: ContainerType::Unset,
                operator: Operator {
                    operator_type: OperatorType::Unset,
                    calculation: CalculationOperator::Unset,
                    relation: RelationOperator::Unset,
                    logical: LogicalOperator::Unset,
                },
            };

            result.push(token);

            source_code = source_code[lexeme.len()..].parse().unwrap();

            continue;
        }

        lexeme = match_number(source_code.clone());
        if lexeme.len() > 0 {
            let token = Token {
                token_type: TokenType::Number,
                number: lexeme.clone(),
                string: "".to_string(),
                identifier: "".to_string(),
                keyword: KeywordType::Unset,
                container: ContainerType::Unset,
                operator: Operator {
                    operator_type: OperatorType::Unset,
                    calculation: CalculationOperator::Unset,
                    relation: RelationOperator::Unset,
                    logical: LogicalOperator::Unset,
                },
            };

            result.push(token);

            source_code = source_code[lexeme.len()..].parse().unwrap();

            continue;
        }

        lexeme = match_string(source_code.clone());
        if lexeme.len() > 0 {
            let token = Token {
                token_type: TokenType::String,
                number: "".to_string(),
                string: lexeme.clone(),
                identifier: "".to_string(),
                keyword: KeywordType::Unset,
                container: ContainerType::Unset,
                operator: Operator {
                    operator_type: OperatorType::Unset,
                    calculation: CalculationOperator::Unset,
                    relation: RelationOperator::Unset,
                    logical: LogicalOperator::Unset,
                },
            };

            result.push(token);

            source_code = source_code[(lexeme.len() + 2)..].parse().unwrap();

            continue;
        }

        let container_type = match_container(source_code.clone());
        match container_type {
            ContainerType::Unset => {}
            _ => {
                let token = Token {
                    token_type: TokenType::Container,
                    number: "".to_string(),
                    string: "".to_string(),
                    identifier: "".to_string(),
                    keyword: KeywordType::Unset,
                    container: container_type,
                    operator: Operator {
                        operator_type: OperatorType::Unset,
                        calculation: CalculationOperator::Unset,
                        relation: RelationOperator::Unset,
                        logical: LogicalOperator::Unset,
                    },
                };

                // Add new token
                result.push(token);

                // All containers have only 1 character
                source_code.remove(0);

                continue;
            }
        }

        let operator = match_operator(source_code.clone());
        match operator.operator_type {
            OperatorType::Unset => {}
            _ => {
                let token = Token {
                    token_type: TokenType::Operator,
                    number: "".to_string(),
                    string: "".to_string(),
                    identifier: "".to_string(),
                    keyword: KeywordType::Unset,
                    container: ContainerType::Unset,
                    operator,
                };

                result.push(token);

                match operator.operator_type {
                    OperatorType::Relation => {
                        if operator.relation == RelationOperator::Equal ||
                            operator.relation == RelationOperator::NotEqual ||
                            operator.relation == RelationOperator::LessEqual ||
                            operator.relation == RelationOperator::BiggerEqual {
                            source_code = source_code[2..].parse().unwrap();
                        } else {
                            source_code.remove(0);
                        }
                    }
                    OperatorType::Logical => {
                        if operator.logical == LogicalOperator::And ||
                            operator.logical == LogicalOperator::Or {
                            source_code = source_code[2..].parse().unwrap();
                        } else {
                            source_code.remove(0);
                        }
                    }
                    OperatorType::Scope => {
                        source_code = source_code[2..].parse().unwrap();
                    }
                    _ => {
                        source_code.remove(0);
                    }
                }

                continue;
            }
        }

        lexeme = match_spaces(source_code.clone());
        if lexeme.len() > 0 {
            source_code = source_code[lexeme.len()..].parse().unwrap();

            continue;
        }

        if match_semicolon(source_code.clone()) {
            source_code.remove(0);

            let token = Token {
                token_type: TokenType::Semicolon,
                number: "".to_string(),
                string: "".to_string(),
                identifier: "".to_string(),
                keyword: KeywordType::Unset,
                container: ContainerType::Unset,
                operator: Operator{
                    operator_type: OperatorType::Unset,
                    calculation: CalculationOperator::Unset,
                    relation: RelationOperator::Unset,
                    logical: LogicalOperator::Unset
                }
            };

            result.push(token);

            continue;
        }
    }

    return result;
}
