pub use json::{from_slice, Value};

/// Dummy cast
pub fn to_json(value: Value) -> Option<json::Value> {
    Some(value)
}
