#![recursion_limit="1024"]

#[macro_use]
extern crate helix;
extern crate owned_chars;

use helix::Symbol;
use owned_chars::{OwnedCharsExt, OwnedChars};

// enum Token {
//     Slash,
//     Star(String),
//     LParen,
//     RParen,
//     Dot,
//     Symbol(String),
//     Literal(String),
// }

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

        def next_token(&mut self) -> Option<(Symbol, String)> {
            let character = self.consume_char();
            let next_character = self.peek_char();

            println!("character: {:?}, next_character: {:?}", character, next_character);
            //     let character = chars.next();
            //     let next_character = chars.peek();
            //
            //     None
            // })
            None

            // self.chars.and_then(|chars| None)


            // match self.chars {
            //     Some(ref mut chars) => { None },
            //     None => { None },
            // }


            // if self.chars.is_none() {
            //     None
            // } else {
            //     let  = self.chars.unwrap().as_re;
            //     let c = chars.next();
            //     None
            // }

            // let c = self.chars.next();
            // return None

            // if self.index >= self.pattern.len() {
            //     None
            // } else if self.pattern[self.index] == '/' {
            //     Some((Symbol::from_string("SLASH".to_string()), "/".to_string()))
            // } else {
            //     None
            // }

            // if self.tokens.len() > 0 {
                // let token = self.tokens.remove(0);
                //
                // if token == "" {
                //     Some((Symbol::from_string("SLASH".to_string()), "/".to_string()))
                // } else {
                //     Some((Symbol::from_string("LITERAL".to_string()), token))
                // }
            // } else {
                // None
            // }
        }
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
}
