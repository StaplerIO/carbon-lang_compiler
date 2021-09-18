use crate::shared::token::KeywordType;

// Convert keyword string to token
pub fn match_keyword(identifier: &str) -> KeywordType {
    return match identifier {
        "decl" => KeywordType::KwDeclare,
        "number" => KeywordType::KwNumber,
        "char" => KeywordType::KwChar,
        "str" => KeywordType::KwStr,
        "var" => KeywordType::KwVar,
        "const" => KeywordType::KwConst,
        "export" => KeywordType::KwExport,
        "func" => KeywordType::KwFunc,
        "if" => KeywordType::KwIf,
        "elif" => KeywordType::KwElseIf,
        "else" => KeywordType::KwElse,
        "while" => KeywordType::KwWhile,
        "loop" => KeywordType::KwLoop,
        "switch" => KeywordType::KwSwitch,
        "continue" => KeywordType::KwContinue,
        "break" => KeywordType::KwBreak,
        "return" => KeywordType::KwReturn,
        "call" => KeywordType::KwCall,
        "link" => KeywordType::KwLink,
        "none" => KeywordType::KwNone,
        _ => KeywordType::Unset,
    };
}
