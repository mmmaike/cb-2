use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
    // Keywords
    #[token("bool")]
    KwBoolean,
    #[token("do")]
    KwDo,
    #[token("else")]
    KwElse,
    #[token("float")]
    KwFloat,
    #[token("for")]
    KwFor,
    #[token("if")]
    KwIf,
    #[token("int")]
    KwInt,
    #[token("printf")]
    KwPrintf,
    #[token("return")]
    KwReturn,
    #[token("void")]
    KwVoid,
    #[token("while")]
    KwWhile,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lss,
    #[token(">")]
    Grt,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    // Misceallanous
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    //Pseudo Tokens
    #[regex("[0-9]")]
    Digit,
    #[regex("C1Token::Digit+")]
    Integer,
    #[regex("C1Token::Integer.C1Token::Integer")]
    #[regex(".C1Token::Integer")]
    Float,
    #[regex("[a-zA-Z]")]
    Letter,

    //Term variables
    #[regex("C1Token::Integer")]
    ConstInt,
    #[regex("C1Token::Float([eE]([-+])?C1Token::Integer)")]
    #[regex("C1Token::Integer[eE]([-+])? C1Token::Integer")]
    ConstFloat,
    #[regex("true")]
    #[regex("false")]
    ConstBoolean,
    #[regex(" \" [^\n\"]* \"")]
    ConstString,
    #[regex("C1Token::Letter+ (C1Token::Digit | C1Token::Letter)*")]
    Id,

    //Comments
    #[regex("/* [^/] */")]
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
