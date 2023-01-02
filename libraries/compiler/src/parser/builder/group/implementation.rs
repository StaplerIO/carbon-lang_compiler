use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::builder::function_builder::function_builder_base;
use crate::parser::utils::pair_container;
use crate::shared::ast::action::ActionContent;
use crate::shared::ast::blocks::data::DataType;
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::ast::group::declaration::{Field, GroupDeclarationBlock};
use crate::shared::ast::group::implementation::{FieldGS, FieldImplementation, FunctionImplementation, GroupImplementationBlock, MethodImplementation};
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;
use crate::shared::token::token::TokenContent;
use crate::shared::utils::identifier::Identifier;

pub fn group_implementation_builder(tokens: &Vec<DecoratedToken>, defined_groups: &Vec<GroupDeclarationBlock>) -> Result<(GroupImplementationBlock, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwImplement)
        && tokens[1].content.is_valid_identifier() {
        let body = pair_container(tokens[2..].to_vec());

        let source = defined_groups
            .iter()
            .find(|&x| x.identifier == *tokens[1].content
                                                 .get_data()
                                                 .unwrap()
                                                 .get_data_accessor()
                                                 .unwrap()
                                                 .get_identifier())
            .unwrap();
        let mut result = GroupImplementationBlock::from_declaration(source);

        let mut next_index = 1;
        while next_index < body.len() {
            if body[next_index].content.get_decorated_keyword().is_some() {
                match body[next_index].content.get_decorated_keyword().unwrap() {
                    KeywordType::KwField => {
                        // Build field in the implementation
                        let current_build = group_field_gs_builder_no_check(&body[next_index..].to_vec(), &source.fields);
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            let target = result.fields
                                .iter_mut()
                                .find(|f| f.source == res.0.source)
                                .unwrap();

                            if res.0.get_block.is_some() {
                                target.get_block = res.0.get_block;
                            } else if res.0.set_block.is_some() {
                                target.set_block = res.0.set_block;
                            }

                            next_index += res.1;
                            continue;
                        }
                    }
                    KeywordType::KwMethod => {
                        // Build method in the implementation
                        let current_build = group_method_builder_no_check(&body[next_index..].to_vec());
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            let target = result.methods.iter_mut().find(|m| m.declarator.identifier == res.0.declarator.identifier).unwrap();

                            target.body = res.0.body;
                            next_index += res.1;
                            continue;
                        }
                    }
                    KeywordType::KwFunc => {
                        // Build function in the implementation
                        let current_build = group_func_builder_no_check(&body[next_index..].to_vec());
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            let target = result.functions.iter_mut().find(|f| f.declarator.identifier == res.0.declarator.identifier).unwrap();

                            target.body = res.0.body;
                            next_index += res.1;
                            continue;
                        }
                    }
                    KeywordType::KwDefault => {
                        // Build default data in the implementation
                        let current_build = group_default_builder_no_check(&body[next_index..].to_vec());
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            let target = result.fields.iter_mut().find(|f| f.source.identifier == res.1).unwrap();
                            target.default_value = res.0;
                            next_index += res.2;
                            continue;
                        }
                    }
                    _ => {
                        return Err(GeneralIssue {
                            issues: vec![IssueBase {
                                level: IssueLevel::Error,
                                position: IssuePosition::Parsing,
                                code: "".to_string(),
                                detail: "Unexpected implementation extension".to_string(),
                            }]
                        });
                    }
                }
            }
        }

        return Ok((result, 2 + body.len() + 1));
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

fn group_default_builder_no_check(tokens: &Vec<DecoratedToken>) -> Result<(SimpleExpression, Identifier, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwDefault)
        && tokens[1].content.is_valid_identifier()
        && tokens[2].original_token.content == TokenContent::Operator(Operator::Assignment) {
        let evaluation = assignment_block_builder(&tokens[1..].to_vec());
        if evaluation.is_ok() {
            let result = evaluation.unwrap();
            match result.0.content {
                ActionContent::AssignmentStatement(ref x) => return Ok((x.rhs_eval_expression.clone(), result.0.get_assignment_action().unwrap().lhs_accessor.get_identifier().clone(), result.1 + 1)),
                _ => panic!("Unknown error!")
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

fn group_func_builder_no_check(tokens: &Vec<DecoratedToken>) -> Result<(FunctionImplementation, usize), GeneralIssue<String>> {
    return function_builder_base(tokens, KeywordType::KwFunc);
}

fn group_method_builder_no_check(tokens: &Vec<DecoratedToken>) -> Result<(MethodImplementation, usize), GeneralIssue<String>> {
    return function_builder_base(&tokens, KeywordType::KwMethod);
}

fn group_field_gs_builder_no_check(tokens: &Vec<DecoratedToken>, defined_fields: &Vec<Field>) -> Result<(FieldImplementation, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwField)
        && tokens[1].content.is_valid_identifier() {
        if tokens[2].content.get_decorated_keyword().is_some()
            && tokens[3].original_token.content == TokenContent::Container(ContainerType::Brace) {
            let identifier_result = Identifier::from_tokens(&tokens[1..].to_vec());
            if identifier_result.is_none() {
                return Err(GeneralIssue {
                    issues: vec![IssueBase {
                        level: IssueLevel::Info,
                        position: IssuePosition::Parsing,
                        code: "".to_string(),
                        detail: "Invalid identifier".to_string(),
                    }]
                });
            }
            let id = identifier_result.unwrap();

            let source = defined_fields.iter().find(|&f| f.declarator.identifier == id.0).unwrap();
            let mut result = FieldImplementation {
                source: source.declarator.clone(),
                slot: 0,
                get_block: None,
                set_block: None,
                default_value: SimpleExpression {
                    postfix_expr: vec![],
                    output_type: DataType {
                        data_type_id: Identifier::empty(),
                        is_array: false,
                    },
                },
            };

            let gs = tokens[id.1 + 1].content.get_decorated_keyword().unwrap();
            let gs_behavior_range = pair_container(tokens[(id.1 + 2)..].to_vec());
            let actions_result = action_block_builder(gs_behavior_range[1..].to_vec());
            if actions_result.is_ok() {
                let actions = actions_result.unwrap();

                match gs {
                    KeywordType::KwGet => {
                        if source.has_get {
                            result.get_block = Some(FieldGS { actions });
                        }
                    }
                    KeywordType::KwSet => {
                        if source.has_set {
                            result.get_block = Some(FieldGS { actions });
                        }
                    }
                    _ => {
                        return Err(GeneralIssue {
                            issues: vec![IssueBase {
                                level: IssueLevel::Error,
                                position: IssuePosition::Parsing,
                                code: "0003".to_string(),
                                detail: "Unexpected field extension, only get and set are available".to_string(),
                            }]
                        });
                    }
                }

                return Ok((result, 2 + id.1 + gs_behavior_range.len() + 1));
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
