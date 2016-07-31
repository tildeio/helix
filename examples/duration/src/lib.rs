#[macro_use]
extern crate helix;

declare_types! {
    class Duration {
        struct {
            inner: u32
        }

        def initialize() -> Duration {
            Duration { inner: 0 }
        }

        def inspect(self) -> String {
            format!("{}", self.inner)
        }
    }
}
