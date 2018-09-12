extern crate lalrpop_util;

use nodes::Expression::{self, *};
use parser::ExpressionsParser;
use scanner::Token::{Dot, Literal, Slash, Star, Symbol};
use scanner::{self, Scanner};

type ParseError = lalrpop_util::ParseError<usize, scanner::Token, scanner::NotPossible>;
type ParseResult = Result<Vec<Expression>, ParseError>;

fn parse(s: &'static str) -> ParseResult {
    let scanner = Scanner::new(s.to_string());
    let parser = ExpressionsParser::new();
    parser.parse(scanner)
}

#[test]
fn test_slash() {
    assert_eq!(parse("/").unwrap(), vec![Token(Slash)]);
}

#[test]
fn test_segment() {
    assert_eq!(
        parse("foo").unwrap(),
        vec![Token(Literal("foo".to_string()))]
    );

    assert_eq!(
        parse("/foo").unwrap(),
        vec![Token(Slash), Token(Literal("foo".to_string()))]
    );

    assert_eq!(
        parse("/foo/").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
        ]
    );
}

#[test]
fn test_segments() {
    assert_eq!(
        parse("/foo/bar/baz").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
            Token(Literal("bar".to_string())),
            Token(Slash),
            Token(Literal("baz".to_string())),
        ]
    );

    assert_eq!(
        parse("/foo/bar/baz/").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
            Token(Literal("bar".to_string())),
            Token(Slash),
            Token(Literal("baz".to_string())),
            Token(Slash),
        ]
    );
}

#[test]
fn test_segment_symbol() {
    assert_eq!(
        parse("/foo/:id").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
            Token(Symbol(":id".to_string())),
        ]
    );
}

#[test]
fn test_symbol() {
    assert_eq!(
        parse(":foo").unwrap(),
        vec![Token(Symbol(":foo".to_string()))]
    );

    assert_eq!(
        parse("/:foo").unwrap(),
        vec![Token(Slash), Token(Symbol(":foo".to_string()))]
    );

    assert_eq!(
        parse("/:foo/").unwrap(),
        vec![
            Token(Slash),
            Token(Symbol(":foo".to_string())),
            Token(Slash),
        ]
    );
}

#[test]
fn test_group() {
    assert_eq!(
        parse("(/:foo)").unwrap(),
        vec![Group(vec![Token(Slash), Token(Symbol(":foo".to_string()))])]
    );
}

#[test]
fn test_groups() {
    assert_eq!(
        parse("(/:foo)(/:bar)").unwrap(),
        vec![
            Group(vec![Token(Slash), Token(Symbol(":foo".to_string()))]),
            Group(vec![Token(Slash), Token(Symbol(":bar".to_string()))]),
        ]
    );
}

#[test]
fn test_nested_groups() {
    assert_eq!(
        parse("(/:foo(/:bar))").unwrap(),
        vec![Group(vec![
            Token(Slash),
            Token(Symbol(":foo".to_string())),
            Group(vec![Token(Slash), Token(Symbol(":bar".to_string()))]),
        ])]
    );
}

#[test]
fn test_dot_symbol() {
    assert_eq!(
        parse(".:format").unwrap(),
        vec![Token(Dot), Token(Symbol(":format".to_string()))]
    );
}

#[test]
fn test_dot_literal() {
    assert_eq!(
        parse(".xml").unwrap(),
        vec![Token(Dot), Token(Literal("xml".to_string()))]
    );
}

#[test]
fn test_segment_dot() {
    assert_eq!(
        parse("/foo.:bar").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Dot),
            Token(Symbol(":bar".to_string())),
        ]
    );
}

#[test]
fn test_segment_group_dot() {
    assert_eq!(
        parse("/foo(.:bar)").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Group(vec![Token(Dot), Token(Symbol(":bar".to_string()))]),
        ]
    );
}

#[test]
fn test_segment_group() {
    assert_eq!(
        parse("/foo(/:action)").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Group(vec![Token(Slash), Token(Symbol(":action".to_string()))]),
        ]
    );
}

#[test]
fn test_segment_groups() {
    assert_eq!(
        parse("/foo(/:action)(/:bar)").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Group(vec![Token(Slash), Token(Symbol(":action".to_string()))]),
            Group(vec![Token(Slash), Token(Symbol(":bar".to_string()))]),
        ]
    );
}

#[test]
fn test_segment_nested_groups() {
    assert_eq!(
        parse("/foo(/:action(/:bar))").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Group(vec![
                Token(Slash),
                Token(Symbol(":action".to_string())),
                Group(vec![Token(Slash), Token(Symbol(":bar".to_string()))]),
            ]),
        ]
    );
}

#[test]
fn test_group_followed_by_path() {
    assert_eq!(
        parse("/foo(/:action)/:bar").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Group(vec![Token(Slash), Token(Symbol(":action".to_string()))]),
            Token(Slash),
            Token(Symbol(":bar".to_string())),
        ]
    );
}

#[test]
fn test_star() {
    assert_eq!(
        parse("*foo").unwrap(),
        vec![Token(Star("*foo".to_string()))]
    );

    assert_eq!(
        parse("/*foo").unwrap(),
        vec![Token(Slash), Token(Star("*foo".to_string()))]
    );

    assert_eq!(
        parse("/*foo/").unwrap(),
        vec![Token(Slash), Token(Star("*foo".to_string())), Token(Slash)]
    );

    assert_eq!(
        parse("/foo/*bar").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
            Token(Star("*bar".to_string())),
        ]
    );

    assert_eq!(
        parse("/foo/*bar/").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
            Token(Star("*bar".to_string())),
            Token(Slash),
        ]
    );

    assert_eq!(
        parse("/*foo/bar").unwrap(),
        vec![
            Token(Slash),
            Token(Star("*foo".to_string())),
            Token(Slash),
            Token(Literal("bar".to_string())),
        ]
    );

    assert_eq!(
        parse("/*foo/bar/").unwrap(),
        vec![
            Token(Slash),
            Token(Star("*foo".to_string())),
            Token(Slash),
            Token(Literal("bar".to_string())),
            Token(Slash),
        ]
    );

    assert_eq!(
        parse("/foo/(*bar)").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
            Group(vec![Token(Star("*bar".to_string()))]),
        ]
    );

    assert_eq!(
        parse("/foo/(*bar)/").unwrap(),
        vec![
            Token(Slash),
            Token(Literal("foo".to_string())),
            Token(Slash),
            Group(vec![Token(Star("*bar".to_string()))]),
            Token(Slash),
        ]
    );

    assert_eq!(
        parse("/(*foo)/bar").unwrap(),
        vec![
            Token(Slash),
            Group(vec![Token(Star("*foo".to_string()))]),
            Token(Slash),
            Token(Literal("bar".to_string())),
        ]
    );

    assert_eq!(
        parse("/(*foo)/bar/").unwrap(),
        vec![
            Token(Slash),
            Group(vec![Token(Star("*foo".to_string()))]),
            Token(Slash),
            Token(Literal("bar".to_string())),
            Token(Slash),
        ]
    );
}

#[test]
fn test_or() {
    assert_eq!(
        parse("a|b").unwrap(),
        vec![Or(vec![
            Token(Literal("a".to_string())),
            Token(Literal("b".to_string())),
        ])]
    );

    assert_eq!(
        parse("a|b|c").unwrap(),
        vec![Or(vec![
            Token(Literal("a".to_string())),
            Token(Literal("b".to_string())),
            Token(Literal("c".to_string())),
        ])]
    );

    assert_eq!(
        parse("(a|b)|c").unwrap(),
        vec![Or(vec![
            Group(vec![Or(vec![
                Token(Literal("a".to_string())),
                Token(Literal("b".to_string())),
            ])]),
            Token(Literal("c".to_string())),
        ])]
    );

    assert_eq!(
        parse("a|(b|c)").unwrap(),
        vec![Or(vec![
            Token(Literal("a".to_string())),
            Group(vec![Or(vec![
                Token(Literal("b".to_string())),
                Token(Literal("c".to_string())),
            ])]),
        ])]
    );

    assert_eq!(
        parse("*a|(b|c)").unwrap(),
        vec![Or(vec![
            Token(Star("*a".to_string())),
            Group(vec![Or(vec![
                Token(Literal("b".to_string())),
                Token(Literal("c".to_string())),
            ])]),
        ])]
    );

    assert_eq!(
        parse("*a|(:b|c)").unwrap(),
        vec![Or(vec![
            Token(Star("*a".to_string())),
            Group(vec![Or(vec![
                Token(Symbol(":b".to_string())),
                Token(Literal("c".to_string())),
            ])]),
        ])]
    );
}

#[test]
fn parse_invalid_paths() {
    assert!(parse("").is_err());
    // assert!(parse("//").is_err());
    // assert!(parse("//foo").is_err());
    // assert!(parse("//foo/").is_err());
    // assert!(parse("//foo//").is_err());
    // assert!(parse("/foo//bar").is_err());
    // assert!(parse("/foo//bar///").is_err());
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
