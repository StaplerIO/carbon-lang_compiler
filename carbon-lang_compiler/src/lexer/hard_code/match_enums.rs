use crate::shared::token::KeywordType;

pub fn match_keyword(identifier: String) -> KeywordType {
    return match identifier.as_str() {
        "decl" => KeywordType::KwDeclare,
        "decimal" => KeywordType::KwDecimal,
        "int" => KeywordType::KwInt,
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
        "none" => KeywordType::KwNone,
        _ => KeywordType::Unset
    };
}
