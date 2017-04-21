#[macro_use]
extern crate helix_runtime;

declare_types! {
    class Calculator {
        def add(lhs: f64, rhs: f64) -> f64 {
            Adder::new(lhs).call(rhs)
        }

        def multiply(lhs: f64, rhs: f64) -> f64 {
            Multiplier::new(lhs).call(rhs)
        }
    }

    class Adder {
        struct {
            lhs: f64
        }

        def initialize(helix, value: f64) {
            Adder { helix, lhs: value }
        }

        def call(&self, rhs: f64) -> f64 {
            self.lhs + rhs
        }
    }

    class Multiplier {
        struct {
            lhs: f64
        }

        def initialize(helix, value: f64) {
            Multiplier { helix, lhs: value }
        }

        def call(&self, rhs: f64) -> f64 {
            self.lhs * rhs
        }
    }
}
