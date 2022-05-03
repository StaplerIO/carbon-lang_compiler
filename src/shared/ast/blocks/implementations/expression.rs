use crate::shared::ast::blocks::expression::{ExprDataTerm, ExprDataTermType};
use crate::shared::ast::decorated_token::DataToken;

impl ExprDataTerm {
    pub fn from_data_token(data: DataToken) -> ExprDataTerm {
        return match data {
            DataToken::Number(x) => ExprDataTerm {
                data_type: ExprDataTermType::Number,
                number: Option::from(x),
                string: None,
                identifier: None,
                function_call: None,
                type_name: None,
            },
            DataToken::String(x) => ExprDataTerm {
                data_type: ExprDataTermType::String,
                number: None,
                string: Option::from(x),
                identifier: None,
                function_call: None,
                type_name: None,
            },
            DataToken::Identifier(x) => ExprDataTerm {
                data_type: ExprDataTermType::Identifier,
                number: None,
                string: None,
                identifier: Option::from(x),
                function_call: None,
                type_name: None,
            },
            DataToken::Typename(x) => ExprDataTerm {
                data_type: ExprDataTermType::Typename,
                number: None,
                string: None,
                identifier: None,
                function_call: None,
                type_name: Option::from(x),
            },
        };
    }
}
