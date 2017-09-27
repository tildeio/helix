#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

ruby! {
    class Calculator {
        def add(lhs: f64, rhs: f64) -> f64 {
            Adder::new(lhs).call(rhs)
        }

        def multiply(lhs: f64, rhs: f64) -> f64 {
            Multiplier::new(lhs).call(rhs)
        }

        def divide(lhs: f64, rhs: f64) -> Result<f64, &'static str> {
            Divider::new(lhs).call(rhs)
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

    class Divider {
        struct {
            lhs: f64
        }

        def initialize(helix, value: f64) {
            Divider { helix, lhs: value }
        }

        def call(&self, rhs: f64) -> Result<f64, &'static str> {
            if rhs == 0f64 {
                Err("Division by zero")
            } else {
                Ok(self.lhs / rhs)
            }
        }
    }
}
