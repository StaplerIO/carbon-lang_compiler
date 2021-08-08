use crate::lexer::hard_code::lex_rule::*;
use crate::lexer::hard_code::match_enums::match_keyword;
use crate::shared::token::{ContainerType, KeywordType, OperatorType, Token, TokenType};

/**
 * Regular expression sequence
 * Number: `\d+(\.\d+)?`
 * String: `"[^"]*"`
 * Identifier: `[a-zA-Z_]([a-zA-Z_0-9])*`
 */

// Support code without comments only
pub fn tokenize(mut source_code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    while source_code.len() > 0 {
        #[allow(unused_assignments)]
            let mut lexeme = String::new();

        if match_semicolon(source_code.clone()) {
            source_code.remove(0);

            let token = Token {
                token_type: TokenType::Semicolon,
                number: None,
                string: None,
                identifier: None,
                keyword: None,
                container: None,
                operator: None,
            };

            result.push(token);

            continue;
        }

        lexeme = match_identifier(source_code.clone());
        if lexeme.len() > 0 {
            // Try match keyword
            let keyword = match_keyword(lexeme.clone());
            if keyword != KeywordType::Unset {
                let token = Token {
                    token_type: TokenType::Keyword,
                    number: None,
                    string: None,
                    identifier: Option::from(lexeme.clone()),
                    keyword: Option::from(keyword),
                    container: None,
                    operator: None,
                };

                result.push(token);
            } else {
                let token = Token {
                    token_type: TokenType::Identifier,
                    number: None,
                    string: None,
                    identifier: Option::from(lexeme.clone()),
                    keyword: None,
                    container: None,
                    operator: None,
                };

                result.push(token);
            }

            source_code = source_code[lexeme.len()..].parse().unwrap();

            continue;
        }

        lexeme = match_number(source_code.clone());
        if lexeme.len() > 0 {
            let token = Token {
                token_type: TokenType::Number,
                number: Option::from(lexeme.clone()),
                string: None,
                identifier: None,
                keyword: None,
                container: None,
                operator: None,
            };

            result.push(token);

            source_code = source_code[lexeme.len()..].parse().unwrap();

            continue;
        }

        lexeme = match_string(source_code.clone());
        if lexeme.len() > 0 {
            let token = Token {
                token_type: TokenType::String,
                number: None,
                string: Option::from(lexeme.clone()),
                identifier: None,
                keyword: None,
                container: None,
                operator: None,
            };

            result.push(token);

            source_code = source_code[(lexeme.len() + 2)..].parse().unwrap();

            continue;
        }

        let container_type = match_container(source_code.clone());
        if container_type != ContainerType::Unset {
            let token = Token {
                token_type: TokenType::Container,
                number: None,
                string: None,
                identifier: None,
                keyword: None,
                container: Option::from(container_type),
                operator: None,
            };

            // Add new token
            result.push(token);

            // All containers have only 1 character
            source_code.remove(0);

            continue;
        }

        let operator_result = match_operator(source_code.clone());
        if operator_result.0.operator_type != OperatorType::Unset {
            let token = Token {
                token_type: TokenType::Operator,
                number: None,
                string: None,
                identifier: None,
                keyword: None,
                container: None,
                operator: Option::from(operator_result.0),
            };

            result.push(token);

            source_code = source_code[(operator_result.1)..].parse().unwrap();

            continue;
        }

        lexeme = match_spaces(source_code.clone());
        if lexeme.len() > 0 {
            source_code = source_code[lexeme.len()..].parse().unwrap();

            continue;
        }
    }

    return result;
}
