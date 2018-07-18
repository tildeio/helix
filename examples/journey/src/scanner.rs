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
                let character = self.consume_char().unwrap();

                ident.push(character);

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

#[derive(Debug)]
pub enum NotPossible {}

impl Iterator for Scanner {
    type Item = Result<Token, NotPossible>;

    fn next(&mut self) -> Option<Self::Item> {
        use self::Token::*;

        self.peek_char().map(|character| match character {
            '/' => {
                self.consume_char();
                Ok(Slash)
            },
            '(' => {
                self.consume_char();
                Ok(LParen)
            },
            ')' => {
                self.consume_char();
                Ok(RParen)
            },
            '.' => {
                self.consume_char();
                Ok(Dot)
            },
            '|' => {
                self.consume_char();
                Ok(Or)
            },
            '*' => {
                let ident = self.consume_symbol_or_star().unwrap();
                Ok(Star(ident))
            },
            ':' => {
                let ident = self.consume_symbol_or_star().unwrap();
                Ok(Symbol(ident))
            },
            _   => {
                let ident = self.consume_literal().unwrap();
                Ok(Literal(ident))
            },
        })
    }
}
