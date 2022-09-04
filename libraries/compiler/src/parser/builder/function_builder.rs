use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::utils::{pair_container, split_comma_expression};
use crate::shared::ast::blocks::function::{Function, FunctionDeclarator};
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenContent};
use crate::shared::ast::parameter::Parameter;
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::token::TokenContent;
use crate::shared::utils::identifier::Identifier;

// Minimum: decl func <name> () [<typename>] {} : 11 tokens
// Set <typename> as "none" to return nothing
pub fn function_builder(tokens: &Vec<DecoratedToken>) -> Result<(Function, usize), GeneralIssue<String>> {
    if tokens.len() >= 10 {
        if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwDeclare) {
            let result = function_builder_base(&tokens[1..].to_vec(), KeywordType::KwFunc);
            if result.is_ok() {
                let val = result.unwrap();
                return Ok((val.0, val.1 + 1));
            }

            return result;
        }
    }

    return Err(GeneralIssue {
        issues: vec![IssueBase {
            level: IssueLevel::Info,
            position: IssuePosition::Parsing,
            code: "".to_string(),
            detail: "".to_string(),
        }]
    });
}

pub fn function_builder_base(tokens: &Vec<DecoratedToken>, leading_keyword: KeywordType) -> Result<(Function, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(leading_keyword) {
        let declarator_build_result = bare_function_declarator_builder(&tokens[1..].to_vec());
        if declarator_build_result.is_ok() {
            let declarator_result = declarator_build_result.unwrap();
            let mut result = Function {
                declarator: declarator_result.0,
                body: vec![],
            };

            // Build argument list
            let argument_raw_array = pair_container(tokens[2..].to_vec());
            result.declarator.parameters = parameter_array_builder(
                argument_raw_array[1..].to_vec(),
            );

            // Build ActionBlock
            let mut current_index = 1 + declarator_result.1;
            if tokens[current_index].content.get_container().is_some() {
                if *tokens[current_index].content.get_container().unwrap() == ContainerType::Brace {
                    let action_block_area =
                        pair_container(tokens[current_index..].to_vec());
                    result.body = action_block_builder(
                        action_block_area[1..action_block_area.len()].to_vec(),
                    ).unwrap();
                    current_index += action_block_area.len();

                    return Ok((result, current_index + 1));
                }
            }
        }
    }

    return Err(GeneralIssue {
        issues: vec![IssueBase {
            level: IssueLevel::Info,
            position: IssuePosition::Parsing,
            code: "".to_string(),
            detail: "".to_string(),
        }]
    });
}

pub fn bare_function_declarator_builder(tokens: &Vec<DecoratedToken>) -> Result<(FunctionDeclarator, usize), GeneralIssue<String>> {
    if tokens.len() >= 5 {
        if tokens[0].content.is_valid_identifier() && tokens[1].content.get_container().is_some()
        {
            if *tokens[1].content.get_container().unwrap() == ContainerType::Bracket {
                // Build argument list
                let argument_raw_array = pair_container(tokens[1..].to_vec());
                let parameters = parameter_array_builder(argument_raw_array[1..argument_raw_array.len()].to_vec());

                // Build return value
                let mut current_index = 2 + argument_raw_array.len();
                if tokens[current_index].content.get_container().is_some() {
                    if *tokens[current_index].content.get_container().unwrap() == ContainerType::Index {
                        let return_value_area = pair_container(tokens[current_index..].to_vec());
                        let return_type = return_value_type_builder(
                            return_value_area[1..return_value_area.len()].to_vec(),
                        );

                        current_index += return_value_area.len() + 1;

                        return Ok((FunctionDeclarator {
                            identifier: tokens[0].content.get_data().unwrap().get_identifier().unwrap().clone(),
                            return_type,
                            parameters,
                        }, current_index));
                    }
                }
            }
        }
    }

    return Err(GeneralIssue {
        issues: vec![IssueBase {
            level: IssueLevel::Info,
            position: IssuePosition::Parsing,
            code: "".to_string(),
            detail: "".to_string(),
        }]
    });
}

// Need raw argument list
fn parameter_array_builder(tokens: Vec<DecoratedToken>) -> Vec<Parameter> {
    if !tokens.is_empty() {
        let list = split_comma_expression(tokens);

        let mut result: Vec<Parameter> = vec![];
        for declaration in list {
            if declaration.len() == 2 {
                if declaration[0].content.is_valid_identifier() && declaration[1].content.is_valid_identifier() {
                    result.push(Parameter {
                        type_name: declaration[0].content.get_data().unwrap().get_identifier().unwrap().clone(),
                        identifier: declaration[1].content.get_data().unwrap().get_identifier().unwrap().clone(),
                    });
                }
            } else {
                // Issue a compiler error
                break;
            }
        }

        return result;
    }

    return vec![];
}

// Return the typename
fn return_value_type_builder(tokens: Vec<DecoratedToken>) -> Identifier {
    if tokens.len() == 1 {
        if tokens[0].content.is_valid_identifier() {
            return tokens[0].content.get_data().unwrap().get_identifier().unwrap().clone();
        } else if tokens[0].content.eq_entry(&DecoratedTokenContent::DecoratedKeyword(KeywordType::Invalid)) {
            if *tokens[0].content.get_decorated_keyword().unwrap() == KeywordType::KwNone {
                return Identifier::empty();
            }
        }
    }

    panic!("Typename required");
}
