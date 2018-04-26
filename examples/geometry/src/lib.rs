#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

ruby! {
    class Point {
        struct {
            x: f64,
            y: f64
        }

        def initialize(helix, x: f64, y: f64) {
            Point { helix, x, y }
        }

        def x(&self) -> f64 {
            self.x
        }

        def y(&self) -> f64 {
            self.y
        }

        def to_a(&self) -> (f64, f64) {
            (self.x, self.y)
        }
    }
}
