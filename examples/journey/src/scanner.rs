use std;
use owned_chars::{OwnedCharsExt, OwnedChars};

pub enum Token {
    Slash,
    LParen,
    RParen,
    Dot,
    Or,
    Star(String),
    Symbol(String),
    Literal(String),
}

#[derive(Debug)]
pub struct Scanner {
    chars: OwnedChars,
    peeked: Option<char>,
}

impl Scanner {
    pub fn new(pattern: String) -> Scanner {
        Scanner {
            chars: pattern.into_chars(),
            peeked: None,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        use self::Token::*;

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

    fn consume_char(&mut self) -> Option<char> {
        if self.peeked.is_some() {
            std::mem::replace(&mut self.peeked, None)
        } else {
            self.chars.next()
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
