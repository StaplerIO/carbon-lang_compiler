/**
 * Regular expression sequence
 * Number: `\d+(\.\d+)?`
 * String: `"[^"]*"`
 * Identifier: `[a-zA-Z_]([a-zA-Z_0-9])*`
 */

use regex::Regex;

use crate::shared::token::{Token, KeywordType, ContainerType, TokenType, Operator, OperatorType, CalculationOperator, RelationOperator, LogicalOperator};
use crate::lexer::hard_code::lex_rule::*;
use crate::lexer::hard_code::match_enums::match_keyword;
use crate::shared::token::TokenType::Container;

pub fn tokenize(mut source_code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    while source_code.len() > 0 {
        let mut match_string = String::new();

        match_string = match_identifier(source_code.clone());
        if match_string.len() > 0 {
            let token = Token {
                token_type: TokenType::Identifier,
                number: "".to_string(),
                string: "".to_string(),
                identifier: match_string.clone(),
                keyword: match_keyword(match_string.clone()),
                container: ContainerType::Unset,
                operator: Operator {
                    operator_type: OperatorType::Unset,
                    calculation: CalculationOperator::Unset,
                    relation: RelationOperator::Unset,
                    logical: LogicalOperator::Unset,
                },
            };

            result.push(token);

            source_code = source_code[match_string.len()..].parse().unwrap();

            continue;
        }

        match_string = match_number(source_code.clone());
        if match_string.len() > 0 {
            let token = Token {
                token_type: TokenType::Number,
                number: match_string.clone(),
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

            source_code = source_code[match_string.len()..].parse().unwrap();

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

                result.push(token);

                source_code = source_code[match_string.len()..].parse().unwrap();
            }

            // continue;
        }

        match_string = match_spaces(source_code.clone());
        if match_string.len() > 0 {
            source_code = source_code[match_string.len()..].parse().unwrap();

            continue;
        }
    }

    return result;
}
