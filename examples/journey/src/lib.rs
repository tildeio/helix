#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

use helix::Symbol;

ruby! {
    #[ruby_name="RustJourneyScanner"]
    class Scanner {
        struct {
            pattern: String,
            index: usize,
        }

        def initialize(helix) {
            Scanner {
                helix,
                pattern: "".to_string(),
                index: 0
            }
        }

        def scan_setup(&mut self, pattern: String) {
            self.pattern = pattern;
            self.index = 0;
        }

        def next_token(&mut self) -> Option<(Symbol, String)> {
            None
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
