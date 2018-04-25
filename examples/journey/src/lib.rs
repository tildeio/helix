#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

use helix::Symbol;

ruby! {
    #[ruby_name="RustJourneyScanner"]
    class Scanner {
        struct {
            count: u32,
        }

        def initialize(helix) {
            Scanner { helix, count: 0 }
        }

        def scan_setup(&self, _pattern: String) {

        }

        def next_token(&mut self) -> Option<(Symbol, String)> {
            self.count += 1;

            if self.count == 1 {
                Some((Symbol::from_string("SLASH".to_string()), "/".to_string()))
            } else {
                None
            }
        }
    }
}
