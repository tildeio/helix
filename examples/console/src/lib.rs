#![recursion_limit="1024"]

#[macro_use]
extern crate helix;
use helix::{sys,FromRuby};

ruby! {
    #[derive(Debug)]
    class Console {
        def log(&self, string: String) {
            println!("{}", string);
        }

        def log_lines(&self, lines: Vec<String>) {
            for l in lines { self.log(l) }
        }

        def inspect(&self) {
            println!("{:?}", self)
        }

        def hello(&self) {
            self.log(String::from("hello"));
        }

        def loglog(&self, string1: String, string2: String) {
            println!("{} {}", string1, string2);
        }

        def log_if(&self, string: String, condition: bool) {
            if condition { self.log(string) };
        }

        def colorize(&self, string: String) -> String {
            format!("\x1B[0;31;49m{}\x1B[0m", string)
        }

        def colorize_lines(&self, lines: Vec<String>) -> Vec<String> {
            lines.into_iter().map(|l| self.colorize(l) ).collect()
        }

        def is_red(&self, string: String) -> bool {
            string.starts_with("\x1B[0;31;49m") && string.ends_with("\x1B[0m")
        }

        def raise(&self) -> Result<(), helix::Error> {
            raise!("raised from Rust with `raise`");
        }

        def raise_panic(&self) {
            raise_panic!("raised from Rust with `raise_panic`");
        }

        def panic(&self) {
            panic!("raised from Rust with `panic`");
        }

        def behave_badly(&self) {
            ruby_funcall!(sys::rb_cObject, "does_not_exist", String::from("one"));
        }

        def call_ruby(&self) -> String {
            let a = ruby_funcall!(sys::rb_cObject, "name"); // No arg
            let b = ruby_funcall!(sys::rb_cObject, "is_a?", sys::rb_cObject); // One arg
            let c = ruby_funcall!(sys::rb_cObject, "respond_to?", String::from("inspect"), true); // Two args
            format!("{:?}, {:?}, {:?}", String::from_ruby_unwrap(a), bool::from_ruby_unwrap(b), bool::from_ruby_unwrap(c))
        }
    }
}
