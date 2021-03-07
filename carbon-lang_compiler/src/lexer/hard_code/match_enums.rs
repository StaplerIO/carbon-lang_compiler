use crate::shared::token::KeywordType;
use crate::shared::token::TokenType::Keyword;
use crate::shared::token::KeywordType::{KwStr, KwVar, KwDecimal, KwInt, KwChar, KwConst, KwExport, KwFunc, KwIf, KwElseIf, KwElse, KwWhile, KwLoop, KwFor, KwSwitch, KwContinue, KwBreak, KwReturn, KwCall, Unset};

pub fn match_keyword(identifier: String) -> KeywordType {
    return match identifier.as_str() {
        "decimal" => KwDecimal,
        "int" => KwInt,
        "char" => KwChar,
        "str" => KwStr,
        "var" => KwVar,
        "const" => KwConst,
        "export" => KwExport,
        "func" => KwFunc,
        "if" => KwIf,
        "elif" => KwElseIf,
        "else" => KwElse,
        "while" => KwWhile,
        "loop" => KwLoop,
        "for" => KwFor,
        "switch" => KwSwitch,
        "continue" => KwContinue,
        "break" => KwBreak,
        "return" => KwReturn,
        "call" => KwCall,
        _ => Unset
    };
}
