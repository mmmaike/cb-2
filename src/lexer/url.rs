use logos::{Lexer, Logos};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    #[regex("<a.*href=\"https?://www.*\".*</a", extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(
        "([^A-Z][^a-z][^0-9][^-\\._~:/\\?\\#@!$&'\\(\\)\\*\\+,;=<>])",
        logos::skip
    )]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let _lexslice = lex.slice();

    let linkurl = LinkUrl(String::from("dummy.de"));
    let linktext = LinkText(String::from("dummylinktext"));
    ((linkurl), (linktext))
}
