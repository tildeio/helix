#![recursion_limit="1024"]

#[macro_use]
extern crate helix;
extern crate docopt;

use docopt::ArgvMap;
use helix::{ToRuby, ToRubyResult};

ruby! {
    class Docopt {
        struct {
            options: ArgvMap,
        }

        #[ruby_visibility=unexported]
        def initialize(helix, options: ArgvMap) {
            Docopt { helix, options }
        }

        def parse(usage: String, argv: Vec<String>) -> Result<Docopt, String> {
            let result = docopt::Docopt::new(usage)
                .and_then(|d| d.help(false).argv(argv.into_iter()).parse());

            match result {
                Ok(args) => Ok(Docopt::new(args)),
                Err(error) => match error {
                    docopt::Error::WithProgramUsage(e, msg) => {
                        Err(format!("{}\n\n{}\n", e, msg))
                    },
                    e => {
                        Err(format!("{}", e))
                    }
                }
            }
        }

        #[ruby_name="[]"]
        def get(&self, key: String) -> ToRubyResult {
            match self.options.map.find(&key) {
                None => ().to_ruby(),
                Some(value) => match *value {
                    docopt::Value::Counted(uint) => uint.to_ruby(),
                    docopt::Value::Plain(None) => ().to_ruby(),
                    ref plain @ docopt::Value::Plain(Some(_)) => plain.as_str().to_ruby(),
                    ref switch @ docopt::Value::Switch(_) => switch.as_bool().to_ruby(),
                    ref list @ docopt::Value::List(_) => list.as_vec().to_ruby()
                },
            }
        }
    }
}
