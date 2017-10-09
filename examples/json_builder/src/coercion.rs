extern crate serde_json;

use super::{JsonValue, JsonBuilder};
use helix::{FromRuby, CheckResult, ToRuby, ToRubyResult};
use helix::sys::VALUE;
use std::collections::HashMap;

pub enum CheckedJsonValue {
    Null,
    Boolean(<bool as FromRuby>::Checked),
    Integer(<i64 as FromRuby>::Checked),
    Float(f64),
    String(<String as FromRuby>::Checked),
    Array(<Vec<JsonValue> as FromRuby>::Checked),
    Object(<HashMap<String, JsonValue> as FromRuby>::Checked),
    Nested(<JsonBuilder as FromRuby>::Checked)
}

impl FromRuby for JsonValue {
    type Checked = CheckedJsonValue;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedJsonValue> {
        if let Ok(_) = <()>::from_ruby(value) {
            Ok(CheckedJsonValue::Null)
        } else if let Ok(checked) = bool::from_ruby(value) {
            Ok(CheckedJsonValue::Boolean(checked))
        } else if let Ok(checked) = i64::from_ruby(value) {
            Ok(CheckedJsonValue::Integer(checked))
        } else if let Ok(checked) = f64::from_ruby(value) {
            let float = f64::from_checked(checked);

            if float.is_normal() {
                Ok(CheckedJsonValue::Float(float))
            } else {
                type_error!(format!("Cannot convert {} into a JSON number", float))
            }
        } else if let Ok(checked) = String::from_ruby(value) {
            Ok(CheckedJsonValue::String(checked))
        } else if let Ok(checked) = Vec::<JsonValue>::from_ruby(value) {
            Ok(CheckedJsonValue::Array(checked))
        } else if let Ok(checked) = HashMap::<String, JsonValue>::from_ruby(value) {
            Ok(CheckedJsonValue::Object(checked))
        } else if let Ok(checked) = JsonBuilder::from_ruby(value) {
            Ok(CheckedJsonValue::Nested(checked))
        } else {
            type_error!(value, "a JSON value")
        }
    }

    fn from_checked(checked: CheckedJsonValue) -> JsonValue {
        match checked {
            CheckedJsonValue::Null => JsonValue::Null,
            CheckedJsonValue::Boolean(c) => JsonValue::Boolean(FromRuby::from_checked(c)),
            CheckedJsonValue::Integer(c) => JsonValue::Integer(FromRuby::from_checked(c)),
            CheckedJsonValue::Float(c) => JsonValue::Float(c),
            CheckedJsonValue::String(c) => JsonValue::String(FromRuby::from_checked(c)),
            CheckedJsonValue::Array(c) => JsonValue::Array(FromRuby::from_checked(c)),
            CheckedJsonValue::Object(c) => JsonValue::Object(FromRuby::from_checked(c)),
            CheckedJsonValue::Nested(c) => JsonValue::Object(JsonBuilder::from_checked(c).to_hash_map())
        }
    }
}

impl ToRuby for JsonValue {
    fn to_ruby(self) -> ToRubyResult {
        match self {
            JsonValue::Null => ().to_ruby(),
            JsonValue::Boolean(v) => v.to_ruby(),
            JsonValue::Integer(v) => v.to_ruby(),
            JsonValue::Float(v) => v.to_ruby(),
            JsonValue::String(v) => v.to_ruby(),
            JsonValue::Array(v) => v.to_ruby(),
            JsonValue::Object(v) => v.to_ruby(),
        }
    }
}

use serde_json::{Value, Number};

pub trait ToSerde {
    fn to_serde(self) -> Value;
}

impl ToSerde for JsonValue {
    fn to_serde(self) -> Value {
        match self {
            JsonValue::Null => Value::Null,
            JsonValue::Boolean(v) => Value::Bool(v),
            JsonValue::Integer(v) => Value::Number(Number::from(v)),
            JsonValue::Float(v) => Value::Number(Number::from_f64(v).unwrap()),
            JsonValue::String(v) => Value::String(v),
            JsonValue::Array(v) => v.to_serde(),
            JsonValue::Object(v) => v.to_serde(),
        }
    }
}

impl ToSerde for Vec<JsonValue> {
    fn to_serde(self) -> Value {
        Value::Array(self.into_iter().map(|v| v.to_serde()).collect())
    }
}

impl ToSerde for HashMap<String, JsonValue> {
    fn to_serde(self) -> Value {
        Value::Object(self.into_iter().map(|(k,v)| (k, v.to_serde())).collect())
    }
}
