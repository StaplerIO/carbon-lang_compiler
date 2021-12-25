use crate::lexer::lex_rules::container::match_container;
use crate::lexer::lex_rules::identifier::match_identifier;
use crate::lexer::lex_rules::keyword::match_keyword;
use crate::lexer::lex_rules::number::match_number;
use crate::lexer::lex_rules::operator::match_operator;
use crate::lexer::lex_rules::semicolon::match_semicolon;
use crate::lexer::lex_rules::space::match_spaces;
use crate::lexer::lex_rules::string::match_string;
use crate::shared::token::{ContainerType, KeywordType, OperatorType, Token};

/**
 * ## Regular expression sequence for lexing source code
 * - Number: `\d+(\.\d+)?`
 * - String: `"[^"]*"`
 * - Identifier: `[a-zA-Z_]([a-zA-Z_0-9])*`
 */

// Support code without comments only
pub fn tokenize(mut source_code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    while source_code.len() > 0 {
        #[allow(unused_assignments)]
        let mut lexeme = String::new();

        if match_semicolon(source_code.as_str()) {
            result.push(Token::new_semicolon());

            source_code.remove(0);
            continue;
        }

        lexeme = match_identifier(source_code.as_str());
        if lexeme.len() > 0 {
            // Try match keyword
            let keyword = match_keyword(lexeme.as_str());
            if keyword != KeywordType::Unset {
                result.push(Token::new_keyword(keyword));
            } else {
                result.push(Token::new_identifier(lexeme.clone()));
            }

            source_code = source_code[lexeme.len()..].parse().unwrap();
            continue;
        }

        lexeme = match_number(source_code.as_str());
        if lexeme.len() > 0 {
            result.push(Token::new_number(lexeme.clone()));

            source_code = source_code[lexeme.len()..].parse().unwrap();
            continue;
        }

        lexeme = match_string(source_code.as_str());
        if lexeme.len() > 0 {
            result.push(Token::new_string(lexeme.clone()));

            source_code = source_code[(lexeme.len() + 2)..].parse().unwrap();
            continue;
        }

        let container_type = match_container(source_code.as_str());
        if container_type != ContainerType::Unset {
            // Add new token
            result.push(Token::new_container(container_type));

            // All containers have only 1 character
            source_code.remove(0);
            continue;
        }

        let operator_result = match_operator(source_code.as_str());
        if operator_result.0.operator_type != OperatorType::Unset {
            result.push(Token::new_operator(operator_result.0));

            source_code = source_code[(operator_result.1)..].parse().unwrap();
            continue;
        }

        // Remove spaces
        lexeme = match_spaces(source_code.as_str());
        if !lexeme.is_empty() {
            source_code = source_code[lexeme.len()..].parse().unwrap();
            continue;
        }
    }

    return result;
}
