#[macro_use]
extern crate helix;

declare_types! {
    class Duration {
        struct {
            inner: u32
        }

        def initialize(helix: helix::Metadata) {
            Duration { helix: helix, inner: 100 }
        }

        def inspect(&self) -> String {
            format!("{:?}", self)
        }

        def incr(&mut self) {
            self.inner += 1;
        }
    }
}
