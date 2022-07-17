use crate::shared::token::keyword::KeywordType;
use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::position::Position;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS: HashMap<KeywordType, &'static str> = [
        (KeywordType::KwDeclare, "decl"),
        (KeywordType::KwNumber, "number"),
        (KeywordType::KwChar, "char"),
        (KeywordType::KwStr, "str"),
        (KeywordType::KwBool, "bool"),
        (KeywordType::KwVar, "var"),
        (KeywordType::KwConst, "const"),
        (KeywordType::KwExport, "export"),
        (KeywordType::KwFunc, "func"),
        (KeywordType::KwIf, "if"),
        (KeywordType::KwElseIf, "elif"),
        (KeywordType::KwElse, "else"),
        (KeywordType::KwWhile, "while"),
        (KeywordType::KwLoop, "loop"),
        (KeywordType::KwContinue, "continue"),
        (KeywordType::KwBreak, "break"),
        (KeywordType::KwReturn, "return"),
        (KeywordType::KwCall, "call"),
        (KeywordType::KwLink, "link"),
        (KeywordType::KwNone, "none"),
        (KeywordType::KwAny, "any"),
        (KeywordType::KwTrue, "true"),
        (KeywordType::KwFalse, "false"),
    ]
    .iter()
    .cloned()
    .collect();
}

// Convert keyword string to token
pub fn match_keyword(content: &str, base_pos: usize) -> Token {
    for (&keyword, &keyword_str) in KEYWORDS.iter() {
        if content.starts_with(keyword_str) {
            return Token::new(
                TokenContent::Keyword(keyword),
                Position::new(base_pos, keyword_str.len()),
            );
        }
    }

    return Token::new_invalid();
}
