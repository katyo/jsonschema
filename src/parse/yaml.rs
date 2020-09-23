pub use yaml::{from_slice, Value};

/// Cast YAML value to JSON
pub fn to_json(value: Value) -> Option<json::Value> {
    use Value::*;

    Some(match value {
        Null => json::Value::Null,
        Bool(value) => json::Value::Bool(value),
        Number(value) => json::Value::Number(conv_number(value)?),
        Value::String(value) => json::Value::String(value),
        Value::Sequence(value) => {
            json::Value::Array(value.into_iter().map(to_json).collect::<Option<_>>()?)
        }
        Value::Mapping(value) => {
            json::Value::Object(value.into_iter().map(conv_kv_pair).collect::<Option<_>>()?)
        }
    })
}

fn conv_number(value: yaml::Number) -> Option<json::Number> {
    Some(if value.is_i64() {
        value.as_i64().unwrap().into()
    } else if value.is_u64() {
        value.as_u64().unwrap().into()
    } else {
        let value = value.as_f64().unwrap();
        match json::Number::from_f64(value) {
            Some(value) => value,
            _ => {
                log::error!("Unable to convert number {} from YAML to JSON", value);
                return None;
            }
        }
    })
}

fn conv_kv_pair((key, value): (Value, Value)) -> Option<(String, json::Value)> {
    let key = to_json(key)?;

    let key = match key {
        json::Value::Null => "null".into(),
        json::Value::Bool(value) => value.to_string(),
        json::Value::Number(value) => value.to_string(),
        json::Value::String(value) => value,
        _ => {
            log::error!(
                "Unable to convert YAML mapping key to JSON because only primitives are valid."
            );
            return None;
        }
    };

    let value = to_json(value)?;

    Some((key, value))
}
