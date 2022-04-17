use lazy_static::lazy_static;
use crate::shared::token::token::{Token, TokenContent};
use crate::shared::utils::position::Position;

lazy_static! {
    static ref COMMENT_LEADING_SLASH: &'static str = "//";
}

// Only line comments
pub fn match_comment(content: &str, base_pos: usize) -> Token {
    if content.starts_with(*COMMENT_LEADING_SLASH) {
        let line_break = content.find('\n');
        if line_break.is_some() {
            // If there's a line break, we can just return the line
            let comment_content = &content[2..line_break.unwrap()];
            return Token::new(
                TokenContent::Comment(comment_content.to_string()),
                Position::new(base_pos, line_break.unwrap() + 2),
            );
        } else {
            // If there is no line break, the comment is the whole line (might be the last line of the file)
            let comment_content = &content[2..];
            return Token::new(
                TokenContent::Comment(comment_content.to_string()),
                Position::new(base_pos, content.len() + 2),
            );
        }
    }

    return Token::new_invalid();
}
