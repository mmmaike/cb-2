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

    //Term variables
    #[regex("[0-9]+")]
    ConstInt,
    #[regex("[0-9]*(e|E)[\\+-]?[0-9]+")]
    #[regex("[0-9]*\\.[0-9]+")]
    ConstFloat,
    #[regex("true")]
    #[regex("false")]
    ConstBoolean,
    #[regex("\"([^\"|\n]*)*\"")]
    ConstString,
    #[regex("[a-zA-Z]+([0-9]|[a-zA-Z])*")]
    Id,

    //Whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    //Comments
    #[regex("(/\\*)[^\\*/]*(\\*/)", logos::skip)]
    #[regex("//.*\n", logos::skip)]
    Comments,
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
