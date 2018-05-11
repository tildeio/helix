#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

use helix::Symbol;

ruby! {
    #[ruby_name="RustJourneyScanner"]
    class Scanner {
        struct {
            tokens: Vec<String>,
        }

        def initialize(helix) {
            Scanner { helix, tokens: vec![] }
        }

        def scan_setup(&mut self, pattern: String) {
            self.tokens.clear();
            let iterator = pattern.split('/');
            self.tokens.extend(iterator.map(|s| s.to_string()));
        }

        def next_token(&mut self) -> Option<(Symbol, String)> {
            if self.tokens.len() > 0 {
                let token = self.tokens.remove(0);

                if token == "" {
                    Some((Symbol::from_string("SLASH".to_string()), "/".to_string()))
                } else {
                    Some((Symbol::from_string("LITERAL".to_string()), token))
                }
            } else {
                None
            }
        }
    }
}
