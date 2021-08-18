use crate::shared::ast::decorated_token::{DecoratedToken, DecoratedTokenType};
use crate::shared::ast::blocks::function::Function;
use crate::shared::ast::parameter::Parameter;
use crate::shared::token::{ContainerType, KeywordType};
use crate::parser::utils::{pair_container, split_comma_expression};
use crate::parser::builder::blocks::action_block::action_block_builder;

// Minimum: decl func <name> () [<typename>] {} : 11 tokens
// Set <typename> as "none" to return nothing
pub fn function_builder(tokens: Vec<DecoratedToken>) -> (Option<Function>, isize) {
    if tokens.len() >= 10 {
        if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword &&
            tokens[1].token_type == DecoratedTokenType::DecoratedKeyword &&
            tokens[3].token_type == DecoratedTokenType::Container {
            if tokens[0].keyword.unwrap() == KeywordType::KwDeclare &&
                tokens[1].keyword.unwrap() == KeywordType::KwFunc &&
                tokens[2].is_valid_identifier() &&
                tokens[3].container.unwrap() == ContainerType::Bracket {
                let mut result = Function {
                    name: tokens[2].clone().data.unwrap().identifier.unwrap(),
                    parameters: vec![],
                    body: vec![],
                    return_type: "".to_string()
                };

                // Build argument list
                let argument_raw_array = pair_container(tokens[3..].to_vec());
                result.parameters = parameter_array_builder(argument_raw_array[1..argument_raw_array.len()].to_vec());

                // Build return value
                let mut current_index = 4 + argument_raw_array.len();
                if tokens[current_index].token_type == DecoratedTokenType::Container {
                    if tokens[current_index].container.unwrap() == ContainerType::Index {
                        let return_value_area = pair_container(tokens[current_index..].to_vec());
                        result.return_type = return_value_type_builder(return_value_area[1..return_value_area.len()].to_vec());

                        // Build ActionBlock
                        current_index += return_value_area.len() + 1;
                        if tokens[current_index].token_type == DecoratedTokenType::Container {
                            if tokens[current_index].container.unwrap() == ContainerType::Brace {
                                let action_block_area = pair_container(tokens[current_index..].to_vec());
                                result.body = action_block_builder(action_block_area[1..action_block_area.len()].to_vec());
                                current_index += action_block_area.len();

                                return (Option::from(result), (current_index as isize) + 1);
                            }
                        }
                    }
                }
            }
        }
    }

    return (None, -1);
}

// Need raw argument list
fn parameter_array_builder(tokens: Vec<DecoratedToken>) -> Vec<Parameter> {
    if !tokens.is_empty() {
        let list = split_comma_expression(tokens);

        let mut result: Vec<Parameter> = vec![];
        for declaration in list {
            if declaration.len() == 2 {
                if declaration[0].is_valid_type() && declaration[1].is_valid_identifier() {
                    result.push(Parameter{
                        type_name: declaration[0].clone().data.unwrap().type_name.unwrap(),
                        identifier: declaration[1].clone().data.unwrap().identifier.unwrap()
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
        if tokens[0].is_valid_type() {
            return tokens[0].clone().data.unwrap().type_name.unwrap();
        } else if tokens[0].token_type == DecoratedTokenType::DecoratedKeyword {
            if tokens[0].keyword.unwrap() == KeywordType::KwNone {
                return String::from("");
            }
        }
    }

    panic!("Typename required");
}