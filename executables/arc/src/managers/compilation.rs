use carbon_lang_compiler::lexer::tokenize::tokenize;
use carbon_lang_compiler::parser::decorator::decorate_token;
use carbon_lang_compiler::parser::pipeline::build_whole_file;
use carbon_lang_compiler::shared::ast::decorated_token::DecoratedToken;
use carbon_lang_compiler::shared::ast::package::ParserPackageStructure;
use carbon_lang_compiler::shared::package_generation::data_descriptor::StringConstant;
use crate::log_error;

pub fn token_conversion(source: &str) -> Option<(Vec<DecoratedToken>, Vec<StringConstant>)> {
    let lexical_analysis_result = tokenize(source, true);

    if lexical_analysis_result.is_err() {
        let err = lexical_analysis_result.unwrap_err();
        for item in err.issues {
            log_error(format!("({}) {}", item.code, item.detail).as_str());
        }
        return None;
    }

    return Some(decorate_token(lexical_analysis_result.unwrap()));
}

pub fn parse_tokens(tokens: Vec<DecoratedToken>, entry_function: Option<String>) -> Option<ParserPackageStructure> {
    let tree_result = build_whole_file(tokens, entry_function.unwrap());

    if tree_result.is_err() {
        let err = tree_result.unwrap_err();
        for item in err.issues {
            log_error(format!("({}) {}", item.code, item.detail).as_str());
        }
        return None;
    }

    return Some(tree_result.unwrap());
}
