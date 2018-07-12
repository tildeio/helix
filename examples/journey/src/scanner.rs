use std;
use owned_chars::{OwnedCharsExt, OwnedCharIndices};

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
    chars: OwnedCharIndices,
    peeked: Option<(usize, char)>,
}

impl Scanner {
    pub fn new(pattern: String) -> Scanner {
        Scanner {
            chars: pattern.into_char_indices(),
            peeked: None,
        }
    }

    fn consume_char(&mut self) -> Option<(usize, char)> {
        if self.peeked.is_some() {
            std::mem::replace(&mut self.peeked, None)
        } else {
            self.chars.next()
        }
    }

    fn peek_char(&mut self) -> Option<(usize, char)> {
        self.peeked.or_else(|| {
            self.peeked = self.consume_char();
            self.peeked
        })
    }

    fn consume_symbol_or_star(&mut self) -> Option<(usize, String, usize)> {
        if self.peek_char().is_none() {
            return None
        } else {
            let mut ident = String::new();
            let start = self.peek_char().unwrap().0;
            let mut end;

            loop {
                let (loc, character) = self.consume_char().unwrap();

                ident.push(character);
                end = loc;

                let is_boundary = match self.peek_char() {
                    Some((_, c)) => match c {
                        '_' => false,
                        c   => !c.is_ascii_alphanumeric(),
                    },
                    None => true
                };

                if is_boundary { break; }
            }

            Some((start, ident, end))
        }
    }

    fn consume_literal(&mut self) -> Option<(usize, String, usize)> {
        if self.peek_char().is_none() {
            return None
        } else {
            let mut ident = String::new();
            let start = self.peek_char().unwrap().0;
            let mut end;

            loop {
                let (loc, character) = self.consume_char().unwrap();

                end = loc;

                if character == '\\' {
                    continue;
                } else {
                    ident.push(character);
                }

                let is_boundary = match self.peek_char() {
                    Some((_, c)) => match c {
                        '/' => true,
                        _   => false,
                    },
                    None => true
                };

                if is_boundary { break; }
            }

            Some((start, ident, end))
        }
    }
}

#[derive(Debug)]
pub enum NotPossible {}

impl Iterator for Scanner {
    type Item = Result<(usize, Token, usize), NotPossible>;

    fn next(&mut self) -> Option<Self::Item> {
        use self::Token::*;

        self.peek_char().map(|(loc, character)| match character {
            '/' => {
                self.consume_char();
                Ok((loc, Slash, loc))
            },
            '(' => {
                self.consume_char();
                Ok((loc, LParen, loc))
            },
            ')' => {
                self.consume_char();
                Ok((loc, RParen, loc))
            },
            '.' => {
                self.consume_char();
                Ok((loc, Dot, loc))
            },
            '|' => {
                self.consume_char();
                Ok((loc, Or, loc))
            },
            '*' => {
                let (start, ident, end) = self.consume_symbol_or_star().unwrap();
                Ok((start, Star(ident), end))
            },
            ':' => {
                let (start, ident, end) = self.consume_symbol_or_star().unwrap();
                Ok((start, Symbol(ident), end))
            },
            _   => {
                let (start, ident, end) = self.consume_literal().unwrap();
                Ok((start, Literal(ident), end))
            },
        })
    }
}
