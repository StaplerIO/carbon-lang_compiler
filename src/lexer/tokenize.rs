use crate::lexer::lex_rules::container::match_container;
use crate::lexer::lex_rules::identifier::match_identifier;
use crate::lexer::lex_rules::keyword::match_keyword;
use crate::lexer::lex_rules::number::match_number;
use crate::lexer::lex_rules::operator::match_operator;
use crate::lexer::lex_rules::semicolon::match_semicolon;
use crate::lexer::lex_rules::space::match_spaces;
use crate::lexer::lex_rules::string::match_string;
use crate::shared::token::token::Token;

/**
 * ## Regular expression sequence for lexing source code
 * - Number: `\d+(\.\d+)?`
 * - String: `"[^"]*"`
 * - Identifier: `[a-zA-Z_]([a-zA-Z_0-9])*`
 */

// Support code without comments only
pub fn tokenize(source_code: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    let mut index: usize = 0;
    while index < source_code.len() {
        let mut token: Token = Token::new_invalid();

        token = match_semicolon(&source_code[index..], index);
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

        token = match_identifier(&source_code[index..], index);
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

        token = match_operator(&source_code[index..], index);
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
            continue;
        }
    }

    return result;
}
