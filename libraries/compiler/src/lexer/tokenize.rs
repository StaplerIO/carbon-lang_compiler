use crate::lexer::lex_rules::comment::match_comment;
use crate::lexer::lex_rules::container::match_container;
use crate::lexer::lex_rules::identifier::match_identifier;
use crate::lexer::lex_rules::keyword::match_keyword;
use crate::lexer::lex_rules::number::match_number;
use crate::lexer::lex_rules::operator::match_operator;
use crate::lexer::lex_rules::semicolon::match_semicolon;
use crate::lexer::lex_rules::space::match_spaces;
use crate::lexer::lex_rules::string::match_string;
use crate::shared::error::general_issue::{FileMatch, GeneralIssue, IssueBase, IssueLevel, IssuePosition};
use crate::shared::error::lexical_analysis_issue::LexicalAnalysisIssue;
use crate::shared::token::token::Token;

/**
 * ## Regular expression sequence for lexing source code
 * - Number: `\d+(\.\d+)?`
 * - String: `"[^"]*"`
 * - Identifier: `[a-zA-Z_]([a-zA-Z_0-9])*`
 */

/**
 * ## Summary
 * Lex a source code file into a sequence of tokens
 * ## Parameters
 * - `source_code`: The source code to lex
 * - `remove_unnecessary_token`: Remove comments and whitespaces when the flag is on
 **/
pub fn tokenize(source_code: &str, remove_unnecessary_token: bool) -> Result<Vec<Token>, GeneralIssue<LexicalAnalysisIssue>> {
    // Error handling variables
    let mut errored: bool = false;
    let mut error_start_index: usize = 0;
    let mut error_list: Vec<IssueBase<LexicalAnalysisIssue>> = vec![];
    // End region

    let mut result: Vec<Token> = vec![];

    let mut index: usize = 0;
    while index < source_code.len() {
        let mut token: Token;

        token = match_comment(&source_code[index..], index);
        if !token.is_invalid() {
            // Check if is in error procedure
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_semicolon(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_identifier(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_keyword(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_operator(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_number(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_string(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_container(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        token = match_spaces(&source_code[index..], index);
        if !token.is_invalid() {
            if errored {
                errored = false;
                error_list.push(IssueBase {
                    level: IssueLevel::Error,
                    position: IssuePosition::LexicalAnalysis,
                    code: "0001".to_string(),
                    detail: LexicalAnalysisIssue {
                        location: FileMatch {
                            file_path: "N/A".to_string(),
                            start_pos: error_start_index,
                            end_pos: index - 1,
                        }
                    },
                });
            }

            index += token.position.length;
            result.push(token);
            continue;
        }

        // Error handling
        if !errored {
            errored = true;
            error_start_index = index;
        }
    }

    if remove_unnecessary_token {
        result.retain(|t| t.get_whitespace().is_none() && t.get_comment().is_none());
    }

    return if error_list.is_empty() {
        Ok(result)
    } else {
        Err(GeneralIssue { issues: error_list })
    };
}
