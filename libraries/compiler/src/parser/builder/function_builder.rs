use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::utils::{pair_container, split_comma_expression};
use crate::shared::ast::blocks::function::Function;
use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenContent};
use crate::shared::ast::parameter::Parameter;
use crate::shared::error::general_error::GeneralError;
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;

// Minimum: decl func <name> () [<typename>] {} : 11 tokens
// Set <typename> as "none" to return nothing
pub fn function_builder(
    tokens: &Vec<DecoratedToken>,
) -> Result<(Function, usize), GeneralError<String>> {
    if tokens.len() >= 10 {
        if tokens[0].content.get_decorated_keyword().is_some()
            && tokens[1].content.get_decorated_keyword().is_some()
            && tokens[3].content.get_container().is_some()
        {
            if *tokens[0].content.get_decorated_keyword().unwrap() == KeywordType::KwDeclare
                && *tokens[1].content.get_decorated_keyword().unwrap() == KeywordType::KwFunc
                && tokens[2].content.is_valid_identifier()
                && *tokens[3].content.get_container().unwrap() == ContainerType::Bracket
            {
                let mut result = Function {
                    name: tokens[2].content.get_data().unwrap().get_identifier().unwrap().clone(),
                    parameters: vec![],
                    body: vec![],
                    return_type: "".to_string(),
                };

                // Build argument list
                let argument_raw_array = pair_container(tokens[3..].to_vec());
                result.parameters = parameter_array_builder(
                    argument_raw_array[1..argument_raw_array.len()].to_vec(),
                );

                // Build return value
                let mut current_index = 4 + argument_raw_array.len();
                if tokens[current_index].content.get_container().is_some()  {
                    if *tokens[current_index].content.get_container().unwrap() == ContainerType::Index {
                        let return_value_area = pair_container(tokens[current_index..].to_vec());
                        result.return_type = return_value_type_builder(
                            return_value_area[1..return_value_area.len()].to_vec(),
                        );

                        // Build ActionBlock
                        current_index += return_value_area.len() + 1;
                        if tokens[current_index].content.get_container().is_some() {
                            if *tokens[current_index].content.get_container().unwrap() == ContainerType::Brace {
                                let action_block_area =
                                    pair_container(tokens[current_index..].to_vec());
                                result.body = action_block_builder(
                                    action_block_area[1..action_block_area.len()].to_vec(),
                                );
                                current_index += action_block_area.len();

                                return Ok((result, current_index + 1));
                            }
                        }
                    }
                }
            }
        }
    }

    return Err(GeneralError {
        code: "-1".to_string(),
        description: None,
    });
}

// Need raw argument list
fn parameter_array_builder(tokens: Vec<DecoratedToken>) -> Vec<Parameter> {
    if !tokens.is_empty() {
        let list = split_comma_expression(tokens);

        let mut result: Vec<Parameter> = vec![];
        for declaration in list {
            if declaration.len() == 2 {
                if declaration[0].content.is_valid_type() && declaration[1].content.is_valid_identifier() {
                    result.push(Parameter {
                        type_name: declaration[0].content.get_data().unwrap().get_typename().unwrap().clone(),
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
fn return_value_type_builder(tokens: Vec<DecoratedToken>) -> String {
    if tokens.len() == 1 {
        if tokens[0].content.is_valid_type() {
            return tokens[0].content.get_data().unwrap().get_typename().unwrap().clone();
        } else if tokens[0].content.eq_entry(&DecoratedTokenContent::DecoratedKeyword(KeywordType::Invalid)) {
            if *tokens[0].content.get_decorated_keyword().unwrap() == KeywordType::KwNone {
                return String::from("");
            }
        }
    }

    panic!("Typename required");
}
