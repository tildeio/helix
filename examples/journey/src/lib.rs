#![recursion_limit="1024"]

#[macro_use]
extern crate helix;
extern crate owned_chars;

use helix::{ToRuby, ToRubyResult};
use owned_chars::{OwnedCharsExt, OwnedChars};

enum Token {
    Slash,
    LParen,
    RParen,
    Dot,
    Or,
    Star(String),
    Symbol(String),
    Literal(String),
}

impl ToRuby for Token {
    fn to_ruby(self) -> ToRubyResult {
        use Token::*;
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

ruby! {
    #[ruby_name="RustJourneyScanner"]
    class Scanner {
        struct {
            chars: Option<OwnedChars>,
            peeked: Option<char>,
        }

        def initialize(helix) {
            Scanner { helix, chars: None, peeked: None }
        }

        def scan_setup(&mut self, pattern: String) {
            self.chars = Some(pattern.into_chars());
            self.peeked = None;
        }

        def next_token(&mut self) -> Option<Token> {
            use Token::*;

            self.peek_char().map(|character| match character {
                '/' => {
                    self.consume_char();
                    Slash
                },
                '(' => {
                    self.consume_char();
                    LParen
                },
                ')' => {
                    self.consume_char();
                    RParen
                },
                '.' => {
                    self.consume_char();
                    Dot
                },
                '|' => {
                    self.consume_char();
                    Or
                },
                '*' => Star(self.consume_symbol_or_star().unwrap()),
                ':' => Symbol(self.consume_symbol_or_star().unwrap()),
                _   => Literal(self.consume_literal().unwrap()),
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
        }

        // How to call a method called `type`?
        // def type(&mut self) {
        //
        // }
    }
}

impl Scanner {
    fn consume_char(&mut self) -> Option<char> {
        if self.peeked.is_some() {
            std::mem::replace(&mut self.peeked, None)
        } else {
            self.chars.as_mut().and_then(|chars| chars.next())
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.peeked.or_else(|| {
            self.peeked = self.consume_char();
            self.peeked
        })
    }

    fn consume_symbol_or_star(&mut self) -> Option<String> {
        if self.peek_char().is_none() {
            return None
        } else {
            let mut ident = String::new();

            loop {
                ident.push(self.consume_char().unwrap());

                let is_boundary = match self.peek_char() {
                    Some(c) => match c {
                        '_' => false,
                        c   => !c.is_ascii_alphanumeric(),
                    },
                    None => true
                };

                if is_boundary { break; }
            }

            Some(ident)
        }
    }

    fn consume_literal(&mut self) -> Option<String> {
        if self.peek_char().is_none() {
            return None
        } else {
            let mut ident = String::new();

            loop {
                let character = self.consume_char().unwrap();

                if character == '\\' {
                    continue;
                } else {
                    ident.push(character);
                }

                let is_boundary = match self.peek_char() {
                    Some(c) => match c {
                        '/' => true,
                        _   => false,
                    },
                    None => true
                };

                if is_boundary { break; }
            }

            Some(ident)
        }
    }
}
