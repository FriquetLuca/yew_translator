use super::StringTemplaterError;
use serde::Serialize;
use std::collections::HashMap;

/// Parse a JSON into a hashmap with it's path concatenated using a dot.
pub fn parse_to_hashmap<T: ?Sized + Serialize>(
    value: &T,
) -> Result<HashMap<String, String>, StringTemplaterError> {
    match serde_json::to_value(value) {
        Ok(serialized) => Ok(encode_json_to_hashmap(&serialized)),
        Err(err) => Err(StringTemplaterError::SerializeError(err.to_string())),
    }
}

/// Parse a JSON into a hashmap with it's path concatenated using a dot.
pub fn encode_json_to_hashmap(value: &serde_json::Value) -> HashMap<String, String> {
    let mut map = HashMap::new();
    flatten("", value, &mut map);
    map
}

fn flatten(prefix: &str, value: &serde_json::Value, map: &mut HashMap<String, String>) {
    match value {
        serde_json::Value::Object(obj) => {
            for (k, v) in obj {
                let new_prefix = if prefix.is_empty() {
                    (*k).clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten(&new_prefix, v, map);
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_prefix = format!("{}.{}", prefix, i);
                flatten(&new_prefix, v, map);
            }
        }
        _ => {
            let _ = match value {
                serde_json::Value::String(s) => map.insert(prefix.to_string(), s.clone()),
                _ => map.insert(prefix.to_string(), value.to_string()),
            };
        }
    }
}
