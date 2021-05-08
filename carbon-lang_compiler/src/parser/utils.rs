use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};

// Return -1 if there's no semicolon token
pub fn find_next_semicolon(tokens: Vec<DecoratedToken>) -> isize {
    for (index, token) in tokens.iter().enumerate() {
        if token.token_type == DecoratedTokenType::StatementEndSign {
            return index as isize;
        }
    }

    return -1;
}
