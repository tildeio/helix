#![recursion_limit="1024"]

#[macro_use]
extern crate helix;
extern crate serde_json;

mod coercion;

use coercion::ToSerde;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone,Debug)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

ruby! {
    pub class JsonBuilder {
        struct {
            entries: HashMap<String, JsonValue>
        }

        def initialize(helix) {
            JsonBuilder { helix, entries: HashMap::new() }
        }

        #[ruby_name="[]="]
        def put(&mut self, key: String, value: JsonValue) {
            self.entries.insert(key, value);
        }

        def to_json(self) -> Result<String, String> {
            serde_json::to_string(&self.entries.to_serde())
                .map_err(|e| e.description().to_string())
        }

        #[ruby_name="to_h"]
        def to_hash_map(self) -> HashMap<String, JsonValue> {
            self.entries
        }
    }
}
