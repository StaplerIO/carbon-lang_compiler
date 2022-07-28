#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
pub enum KeywordType {
    KwNumber,   // number
    KwChar,     // char
    KwStr,      // str
    KwBool,     // bool
    KwDeclare,  // decl
    KwVar,      // var
    KwConst,    // const
    KwExport,   // export
    KwFunc,     // func
    KwIf,       // if
    KwElseIf,   // elif
    KwElse,     // else
    KwWhile,    // while
    KwLoop,     // loop
    KwContinue, // continue
    KwBreak,    // break
    KwReturn,   // return
    KwCall,     // call
    KwLink,     // link
    KwNone,     // none
    KwAny,      // any
    KwTrue,     // true
    KwFalse,    // false
    Invalid,
}
