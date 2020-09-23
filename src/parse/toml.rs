pub use toml::{from_slice, Value};

/// Cast TOML value to JSON
pub fn to_json(value: Value) -> Option<json::Value> {
    use Value::*;

    Some(match value {
        String(value) => json::Value::String(value),
        Integer(value) => json::Value::Number(value.into()),
        Float(value) => json::Value::Number(conv_float(value)?),
        Boolean(value) => json::Value::Bool(value),
        Datetime(value) => json::Value::String(value.to_string()),
        Array(value) => json::Value::Array(value.into_iter().map(to_json).collect::<Option<_>>()?),
        Table(value) => {
            json::Value::Object(value.into_iter().map(conv_kv_pair).collect::<Option<_>>()?)
        }
    })
}

fn conv_float(value: f64) -> Option<json::Number> {
    json::Number::from_f64(value).or_else(|| {
        log::error!("Unable to convert number {} from TOML to JSON", value);
        None
    })
}

fn conv_kv_pair((key, value): (String, Value)) -> Option<(String, json::Value)> {
    let value = to_json(value)?;
    Some((key, value))
}
