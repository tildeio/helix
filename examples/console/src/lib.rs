#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

ruby! {
    class Console {
        def log(&self, string: String) {
            println!("{}", string);
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
    }
}
