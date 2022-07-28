use crate::shared::ast::action::CallAction;
use crate::shared::ast::blocks::expression::{ExprDataTerm, TermContent};
use crate::shared::ast::decorated_token::DataToken;
use crate::shared::token::operator::Operator;

impl TermContent {
    pub fn get_data_term(&self) -> Option<&ExprDataTerm> {
        return match self {
            TermContent::Data(data_term) => Some(&data_term),
            _ => None,
        }
    }

    pub fn get_operator(&self) -> Option<&Operator> {
        return match self {
            TermContent::Operator(operator) => Some(operator),
            _ => None,
        }
    }

    pub fn get_priority(&self) -> Option<bool> {
        return match self {
            TermContent::Priority(p) => Some(*p),
            _ => None,
        }
    }
}

impl ExprDataTerm {
    pub fn from_data_token(d: &DataToken) -> ExprDataTerm {
        return match &d {
            DataToken::Number(x) => ExprDataTerm::Number(x.clone()),
            DataToken::String(x) => ExprDataTerm::String(x.clone()),
            DataToken::Identifier(x) => ExprDataTerm::Identifier(x.clone()),
            _ => panic!("Invalid data token"),
        }
    }

    pub fn get_number(&self) -> Option<&String> {
        return match self {
            ExprDataTerm::Number(number) => return Some(number),
            _ => None,
        };
    }

    pub fn get_string(&self) -> Option<&String> {
        return match self {
            ExprDataTerm::String(string) => return Some(string),
            _ => None,
        };
    }

    pub fn get_identifier(&self) -> Option<&String> {
        return match self {
            ExprDataTerm::Identifier(identifier) => return Some(identifier),
            _ => None,
        };
    }

    pub fn get_function_call(&self) -> Option<&CallAction> {
        return match self {
            ExprDataTerm::FunctionCall(function_call) => return Some(function_call),
            _ => None,
        };
    }

    pub fn get_typename(&self) -> Option<&String> {
        return match self {
            ExprDataTerm::Typename(typename) => return Some(typename),
            _ => None,
        };
    }
}
