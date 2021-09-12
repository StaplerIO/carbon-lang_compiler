use crate::shared::ast::decorated_token::{DataType, DataToken};
use crate::shared::ast::blocks::expression::{ExprDataTerm, ExprDataTermType};

impl ExprDataTerm {
    pub fn from_data_token(data: DataToken) -> ExprDataTerm {
        return match data.data_type {
            DataType::Number => {
                ExprDataTerm {
                    data_type: ExprDataTermType::Number,
                    number: data.number,
                    string: None,
                    identifier: None,
                    function_call: None,
                    type_name: None
                }
            },
            DataType::String => {
                ExprDataTerm {
                    data_type: ExprDataTermType::String,
                    number: None,
                    string: data.string,
                    identifier: None,
                    function_call: None,
                    type_name: None
                }
            },
            DataType::Identifier => {
                ExprDataTerm {
                    data_type: ExprDataTermType::Identifier,
                    number: None,
                    string: None,
                    identifier: data.identifier,
                    function_call: None,
                    type_name: None
                }
            },
            DataType::Type => {
                ExprDataTerm {
                    data_type: ExprDataTermType::TypeName,
                    number: None,
                    string: None,
                    identifier: None,
                    function_call: None,
                    type_name: data.type_name
                }
            }
        }
    }
}
