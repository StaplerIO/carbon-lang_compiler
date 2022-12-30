use crate::parser::builder::function_builder::bare_function_declarator_builder;
use crate::parser::utils::{pair_container, parameter_builder_exact};
use crate::shared::ast::blocks::data::DataDeclarator;
use crate::shared::ast::blocks::function::FunctionDeclarator;
use crate::shared::ast::decorated_token::DecoratedToken;
use crate::shared::ast::group::declaration::{Field, GroupDeclarationBlock, MethodDeclarator};
use crate::shared::error::general_issue::{GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::token::container::ContainerType;
use crate::shared::token::keyword::KeywordType;
use crate::shared::token::operator::Operator;
use crate::shared::token::token::TokenContent;
use crate::shared::utils::identifier::Identifier;

pub fn group_declaration_builder(tokens: &Vec<DecoratedToken>) -> Result<(GroupDeclarationBlock, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwGroup)
        && tokens[1].content.is_valid_identifier() {
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
        let identifier = identifier_result.unwrap();

        let mut result = GroupDeclarationBlock {
            identifier: identifier.0,
            fields: vec![],
            methods: vec![],
            functions: vec![],
        };

        let mut index = 1;
        let body = pair_container(tokens[(1 + identifier.1)..].to_vec());
        while index < body.len() {
            if body[index].content.get_decorated_keyword().is_some() {
                match body[index].content.get_decorated_keyword().unwrap() {
                    KeywordType::KwField => {
                        let current_build = group_field_builder_no_check(&body[index..].to_vec());
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            result.fields.push(res.0);
                            index += res.1;
                            continue;
                        }
                    }
                    KeywordType::KwMethod => {
                        let current_build = group_method_builder_no_check(&body[index..].to_vec());
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            result.methods.push(res.0);
                            index += res.1;
                            continue;
                        }
                    }
                    KeywordType::KwFunc => {
                        let current_build = group_function_builder_no_check(&body[index..].to_vec());
                        if current_build.is_ok() {
                            let res = current_build.unwrap();
                            result.functions.push(res.0);
                            index += res.1;
                            continue;
                        }
                    }
                    _ => {}
                }
            }

            return Err(GeneralIssue {
                issues: vec![IssueBase {
                    level: IssueLevel::Info,
                    position: IssuePosition::Parsing,
                    code: "0001".to_string(),
                    detail: "Unknown property type".to_string(),
                }]
            });
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

fn group_field_builder_no_check(tokens: &Vec<DecoratedToken>) -> Result<(Field, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwField) {
        if tokens[1].content.is_valid_identifier() && tokens[2].content.is_valid_identifier() {
            if tokens[3].original_token.content == TokenContent::Container(ContainerType::Bracket) {
                let declarator_result = parameter_builder_exact(&tokens[1..].to_vec());
                if declarator_result.is_none() {
                    return Err(GeneralIssue {
                        issues: vec![IssueBase {
                            level: IssueLevel::Info,
                            position: IssuePosition::Parsing,
                            code: "".to_string(),
                            detail: "Invalid data declarator".to_string(),
                        }]
                    });
                }
                let declarator = declarator_result.unwrap();

                let mut result = Field {
                    declarator: DataDeclarator {
                        data_type: declarator.type_name.data_type_id,
                        identifier: declarator.identifier,
                        is_array: declarator.type_name.is_array,
                    },
                    has_get: false,
                    has_set: false,
                };

                if tokens[4].original_token.content == TokenContent::Keyword(KeywordType::KwGet) {
                    result.has_get = true;
                } else if tokens[4].original_token.content == TokenContent::Keyword(KeywordType::KwSet) {
                    result.has_set = true;
                }

                if tokens[5].original_token.content == TokenContent::Operator(Operator::Comma) {
                    if result.has_get {
                        if tokens[6].original_token.content == TokenContent::Keyword(KeywordType::KwSet) {
                            result.has_set = true;
                        } else if tokens[6].original_token.content == TokenContent::Keyword(KeywordType::KwGet) {
                            return Err(GeneralIssue {
                                issues: vec![IssueBase {
                                    level: IssueLevel::Info,
                                    position: IssuePosition::Parsing,
                                    code: "0002".to_string(),
                                    detail: "Duplicated feature declaration".to_string(),
                                }]
                            });
                        }
                    } else if result.has_set {
                        if tokens[6].original_token.content == TokenContent::Keyword(KeywordType::KwGet) {
                            result.has_get = true;
                        } else if tokens[6].original_token.content == TokenContent::Keyword(KeywordType::KwSet) {
                            return Err(GeneralIssue {
                                issues: vec![IssueBase {
                                    level: IssueLevel::Info,
                                    position: IssuePosition::Parsing,
                                    code: "0002".to_string(),
                                    detail: "Duplicated feature declaration".to_string(),
                                }]
                            });
                        }
                    }

                    if tokens[7].original_token.content == TokenContent::Container(ContainerType::AntiBracket)
                        && tokens[8].original_token.content == TokenContent::Semicolon {
                        return Ok((result, 9));
                    }
                } else if tokens[5].original_token.content == TokenContent::Container(ContainerType::AntiBracket)
                    && tokens[6].original_token.content == TokenContent::Semicolon {
                    return Ok((result, 7));
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

fn group_method_builder_no_check(tokens: &Vec<DecoratedToken>) -> Result<(MethodDeclarator, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwMethod) {
        let build_result = bare_function_declarator_builder(&tokens[1..].to_vec());
        if build_result.is_ok() {
            let result = build_result.unwrap();

            if tokens[result.1 + 1].original_token.content == TokenContent::Semicolon {
                return Ok((result.0, 1 + result.1 + 1));
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

fn group_function_builder_no_check(tokens: &Vec<DecoratedToken>) -> Result<(FunctionDeclarator, usize), GeneralIssue<String>> {
    if tokens[0].original_token.content == TokenContent::Keyword(KeywordType::KwFunc) {
        let build_result = bare_function_declarator_builder(&tokens[1..].to_vec());
        if build_result.is_ok() {
            let result = build_result.unwrap();

            if tokens[result.1 + 1].original_token.content == TokenContent::Semicolon {
                return Ok((result.0, 1 + result.1 + 1));
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
