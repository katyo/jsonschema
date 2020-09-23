pub use json5::{from_slice, Value};

pub fn to_json(value: Value) -> Option<json::Value> {
    use Value::*;

    match value {}
}
