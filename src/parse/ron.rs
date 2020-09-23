pub use ron::{de::from_bytes as from_slice, Value};

pub fn to_json(value: Value) -> Option<json::Value> {
    Some(match value {
        Value::Bool(value) => json::Value::Bool(value),
        Value::Char(value) => json::Value::String(value.to_string()),
        Value::Number(value) => json::Value::Number(conv_number(value)?),
        Value::String(value) => json::Value::String(value),
        Value::Option(value) => match value {
            Some(value) => to_json(*value)?,
            _ => json::Value::Null,
        },
        Value::Seq(value) => {
            json::Value::Array(value.into_iter().map(to_json).collect::<Option<_>>()?)
        }
        Value::Map(value) => {
            // TODO: Add IntoIterator traits to ROM Map
            json::Value::Object(value.iter().map(conv_kv_pair).collect::<Option<_>>()?)
        }
        Value::Unit => json::Value::Array(Vec::default()),
    })
}

fn conv_number(value: ron::Number) -> Option<json::Number> {
    use ron::Number::*;

    Some(match value {
        Integer(value) => value.into(),
        Float(value) => {
            let value = value.get();
            match json::Number::from_f64(value) {
                Some(value) => value,
                _ => {
                    log::error!("Unable to convert number {} from RON to JSON", value);
                    return None;
                }
            }
        }
    })
}

fn conv_kv_pair((key, value): (&Value, &Value)) -> Option<(String, json::Value)> {
    let key = to_json(key.clone())?;

    let key = match key {
        json::Value::Null => "null".into(),
        json::Value::Bool(value) => value.to_string(),
        json::Value::Number(value) => value.to_string(),
        json::Value::String(value) => value,
        _ => {
            log::error!("Unable to convert RON map key to JSON because only primitives are valid.");
            return None;
        }
    };

    let value = to_json(value.clone())?;

    Some((key, value))
}
