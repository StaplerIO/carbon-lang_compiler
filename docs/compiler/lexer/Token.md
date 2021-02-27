###### Carbon > Compiler > Lexer

# Token

## Basic Literals (Regular Expression)

- Number : `([1-9]\d*\.?\d*)|(0\.\d*[1-9])`
- Identifier : `[a-zA-Z_]([a-zA-Z_0-9])*`
- Operator : Calculate & Relation & Logical
- Container : Brace & Bracket & Index & ~~(Generic)~~
- Keyword : keywords
- String : `[^"]*`
