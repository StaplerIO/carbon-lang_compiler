use crate::lexer::lex_rules::comment::match_comment;
use crate::lexer::lex_rules::container::match_container;
use crate::lexer::lex_rules::identifier::match_identifier;
use crate::lexer::lex_rules::keyword::match_keyword;
use crate::lexer::lex_rules::number::match_number;
use crate::lexer::lex_rules::operator::match_operator;
use crate::lexer::lex_rules::semicolon::match_semicolon;
use crate::lexer::lex_rules::space::match_spaces;
use crate::lexer::lex_rules::string::match_string;
use crate::shared::token::token::{Token, TokenContent};

/**
 * ## Regular expression sequence for lexing source code
 * - Number: `\d+(\.\d+)?`
 * - String: `"[^"]*"`
 * - Identifier: `[a-zA-Z_]([a-zA-Z_0-9])*`
 */

/**
 * ## Summary
 * Lex a source code file into a sequence of tokens
 * ## Parameters
 * - `source_code`: The source code to lex
 * - `remove_unnecessary_token`: Remove comments and whitespaces when the flag is on
**/
pub fn tokenize(source_code: &str, remove_unnecessary_token: bool) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];

    let mut index: usize = 0;
    while index < source_code.len() {
        #[allow(unused_assignments)]
            let mut token: Token = Token::new_invalid();

        token = match_comment(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_semicolon(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_identifier(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_keyword(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_operator(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_number(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_string(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_container(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_spaces(&source_code[index..], index);
        if !token.is_invalid() {
            index += token.position.length;
            result.push(token);
            continue;
        }
    }

    if remove_unnecessary_token {
        let mut index: usize = 0;
        while index < result.len() {
            match result[index].content {
                TokenContent::Comment(_) => {
                    result.remove(index);
                },
                TokenContent::Whitespace(_) => {
                    result.remove(index);
                },
                _ => {
                    index += 1;
                }
            }
        }
    }

    return result;
}
