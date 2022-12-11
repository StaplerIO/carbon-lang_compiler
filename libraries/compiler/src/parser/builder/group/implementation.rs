use crate::parser::builder::blocks::action_block::action_block_builder;
use crate::parser::builder::blocks::assignment::assignment_block_builder;
use crate::parser::builder::function_builder::function_builder_base;
use crate::parser::utils::pair_container;
use crate::shared::ast::action::ActionContent;
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
                                                 .get_identifier()
                                                 .unwrap())
            .unwrap();
        let mut result = GroupImplementationBlock::from_declaration(source);

        let mut next_index = 1;
        while next_index < body.len() {
            if body[next_index].content.get_decorated_keyword().is_some() {
                match body[next_index].content.get_decorated_keyword().unwrap() {
                    KeywordType::KwField => {
                        let current_build = group_field_gs_builder_no_check(&body[next_index..].to_vec(), &source.fields);
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            let target = result.fields.iter_mut().find(|f| f.identifier == res.1).unwrap();

                            if res.0.get_block.is_some() {
                                target.get_block = res.0.get_block;
                            } else if res.0.set_block.is_some() {
                                target.set_block = res.0.set_block;
                            }

                            next_index += res.2;
                            continue;
                        }
                    }
                    KeywordType::KwMethod => {
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
                        let current_build = group_default_builder_no_check(&body[next_index..].to_vec());
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            let target = result.fields.iter_mut().find(|f| f.identifier == res.1).unwrap();
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
                ActionContent::AssignmentStatement(x) => return Ok((x.eval_expression, tokens[1].content.get_data().unwrap().get_identifier().unwrap().clone(), result.1 + 1)),
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

fn group_field_gs_builder_no_check(tokens: &Vec<DecoratedToken>, defined_fields: &Vec<Field>) -> Result<(FieldImplementation, Identifier, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwField)
        && tokens[1].content.is_valid_identifier() {
        if tokens[2].content.get_decorated_keyword().is_some()
            && tokens[3].original_token.content == TokenContent::Container(ContainerType::Brace) {
            let id = tokens[1].content.get_data().unwrap().get_identifier().unwrap();
            let source = defined_fields.iter().find(|&f| f.identifier == *id).unwrap();
            let mut result = FieldImplementation {
                identifier: id.clone(),
                slot: 0,
                get_block: None,
                set_block: None,
                default_value: SimpleExpression { postfix_expr: vec![], output_type: Identifier::empty() },
            };

            let gs = tokens[2].content.get_decorated_keyword().unwrap();
            let gs_behavior_range = pair_container(tokens[3..].to_vec());
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

                return Ok((result, tokens[1].content.get_data().unwrap().get_identifier().unwrap().clone(), 3 + gs_behavior_range.len() + 1));
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
