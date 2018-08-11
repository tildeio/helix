extern crate lalrpop_util;

use parser::*;
use scanner::*;

type ParseError = lalrpop_util::ParseError<usize, Token, NotPossible>;
type ParseResult = Result<Vec<Token>, ParseError>;

fn parse(s: &'static str) -> ParseResult {
    let scanner = Scanner::new(s.to_string());
    let parser = PathParser::new();
    parser.parse(scanner)
}

#[test]
fn parse_slash() {
    assert_eq!(parse("/").unwrap(), vec![Token::Slash]);
}

#[test]
fn parse_literal() {
    assert_eq!(parse("foo").unwrap(), vec![Token::Literal("foo".to_string())]);
}

#[test]
fn parse_simple_path() {
    assert_eq!(parse("/foo").unwrap(), vec![
        Token::Slash,
        Token::Literal("foo".to_string())
    ]);

    assert_eq!(parse("/foo/").unwrap(), vec![
        Token::Slash,
        Token::Literal("foo".to_string()),
        Token::Slash
    ]);
}

#[test]
fn parse_long_path() {
    assert_eq!(parse("/foo/bar/baz").unwrap(), vec![
        Token::Slash,
        Token::Literal("foo".to_string()),
        Token::Slash,
        Token::Literal("bar".to_string()),
        Token::Slash,
        Token::Literal("baz".to_string())
    ]);

    assert_eq!(parse("/foo/bar/baz/").unwrap(), vec![
        Token::Slash,
        Token::Literal("foo".to_string()),
        Token::Slash,
        Token::Literal("bar".to_string()),
        Token::Slash,
        Token::Literal("baz".to_string()),
        Token::Slash
    ]);
}

#[test]
fn parse_star() {
    assert_eq!(parse("*foo").unwrap(), vec![Token::Star("*foo".to_string())]);

    assert_eq!(parse("/*foo").unwrap(), vec![
        Token::Slash,
        Token::Star("*foo".to_string())
    ]);

    assert_eq!(parse("/*foo/").unwrap(), vec![
        Token::Slash,
        Token::Star("*foo".to_string()),
        Token::Slash
    ]);

    assert_eq!(parse("/foo/*bar").unwrap(), vec![
        Token::Slash,
        Token::Literal("foo".to_string()),
        Token::Slash,
        Token::Star("*bar".to_string())
    ]);

    assert_eq!(parse("/foo/*bar/").unwrap(), vec![
        Token::Slash,
        Token::Literal("foo".to_string()),
        Token::Slash,
        Token::Star("*bar".to_string()),
        Token::Slash
    ]);

    assert_eq!(parse("/*foo/bar").unwrap(), vec![
        Token::Slash,
        Token::Star("*foo".to_string()),
        Token::Slash,
        Token::Literal("bar".to_string())
    ]);

    assert_eq!(parse("/*foo/bar/").unwrap(), vec![
        Token::Slash,
        Token::Star("*foo".to_string()),
        Token::Slash,
        Token::Literal("bar".to_string()),
        Token::Slash
    ]);
}

#[test]
fn parse_invalid_paths() {
    assert!(parse("").is_err());
    assert!(parse("//").is_err());
    assert!(parse("//foo").is_err());
    assert!(parse("//foo/").is_err());
    assert!(parse("//foo//").is_err());
    assert!(parse("/foo//bar").is_err());
    assert!(parse("/foo//bar///").is_err());
}

// #[test]
// fn parse_star() {
//     assert_eq!(parse("*foo").unwrap(), Token::Star("*foo".to_string()));
// }
//
// #[test]
// fn parse_unmatched_parens() {
//     assert!(parse("(").is_err());
//     assert!(parse(")").is_err());
// }


// #[test]
// fn parse_path() {
//     let scanner = Scanner::new("/foo/bar".to_string());
//     let parser = TerminalParser::new();
//     let result = parser.parse(scanner);
//
//     assert_eq!(result.unwrap(), vec![
//         Token::Slash,
//         Token::Literal("foo".to_string()),
//         Token::Slash,
//         Token::Literal("bar".to_string())
//     ]);
// }
