#![recursion_limit="1024"]

#[macro_use]
extern crate helix;
extern crate owned_chars;

pub mod scanner;

#[allow(warnings)]
mod parser;
mod nodes;

#[cfg(test)]
mod tests;

ruby! {
    #[ruby_name="RustJourneyScanner"]
    class Scanner {
        struct {
            inner: Option<scanner::Scanner>,
        }

        def initialize(helix) {
            Scanner { helix, inner: None }
        }

        def scan_setup(&mut self, pattern: String) {
            self.inner = Some(scanner::Scanner::new(pattern))
        }

        def next_token(&mut self) -> Option<scanner::Token> {
            self.inner.as_mut().and_then(|s| {
                s.next().map(|r| r.unwrap().1)
            })
        }
    }

    #[ruby_name="RustJourneyParser"]
    class Parser {
        struct { }

        def initialize(helix) {
            Parser { helix }
        }

        def parse(&mut self, _token_string: String) {
            // StarParser::new().parse(&token_string).unwrap().to_string()
        }

        // FIXME: Need to call 'type' on the return value,
        // not on the Parser instance itself.
        // Can't name a function type, using format instead.
        #[ruby_name="type"]
        def format(&mut self) {
        }
    }
}

use helix::{ToRuby, ToRubyResult};

impl ToRuby for scanner::Token {
    fn to_ruby(self) -> ToRubyResult {
        use scanner::Token::*;
        use helix::Symbol as RSymbol;

        match self {
            Slash          => (RSymbol::from_string("SLASH".to_string()), "/").to_ruby(),
            LParen         => (RSymbol::from_string("LPAREN".to_string()), "(").to_ruby(),
            RParen         => (RSymbol::from_string("RPAREN".to_string()), ")").to_ruby(),
            Dot            => (RSymbol::from_string("DOT".to_string()), ".").to_ruby(),
            Or             => (RSymbol::from_string("OR".to_string()), "|").to_ruby(),
            Star(ident)    => (RSymbol::from_string("STAR".to_string()), ident).to_ruby(),
            Symbol(ident)  => (RSymbol::from_string("SYMBOL".to_string()), ident).to_ruby(),
            Literal(ident) => (RSymbol::from_string("LITERAL".to_string()), ident).to_ruby(),
        }
    }
}
