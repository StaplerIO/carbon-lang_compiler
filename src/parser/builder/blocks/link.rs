use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::parser::utils::find_next_semicolon;

pub fn link_statement_builder(tokens: Vec<DecoratedToken>) -> (Option<String>, isize) {
    if tokens.len() >= 3 {
        let next_semicolon_pos = find_next_semicolon(tokens.clone());

        if next_semicolon_pos == 2 {
            if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword && tokens[1].is_valid_identifier() {
                return (tokens[1].clone().data.unwrap().identifier, 2);
            }
        }
    }

    return (None, -1);
}
