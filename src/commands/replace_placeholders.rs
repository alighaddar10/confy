use serde_yaml::Value;

pub fn replace_placeholders(value: Value) -> Value {
    match value {
        Value::String(s) => {
            if s.starts_with("{{env:") && s.ends_with("}}") {
                let var_name = &s["{{env:".len()..s.len() - 2];
                match std::env::var(var_name) {
                    Ok(env_val) => Value::String(env_val),
                    Err(_) => panic!("Environment variable {} is not set", var_name),
                }
            } else if s.starts_with("{{file:") && s.ends_with("}}") {
                let file_path = &s["{{file:".len()..s.len() - 2];
                let file_contents = match std::fs::read_to_string(file_path) {
                    Ok(contents) => contents,
                    Err(e) => panic!("Failed to read file {}: {}", file_path, e),
                };
                match serde_yaml::from_str(&file_contents) {
                    Ok(yaml) => yaml,
                    Err(e) => panic!("Failed to parse YAML from file {}: {}", file_path, e),
                }
            } else {
                Value::String(s)
            }
        }
        Value::Mapping(mapping) => {
            let mut new_map = serde_yaml::Mapping::new();
            for (k, v) in mapping {
                let replaced_value = replace_placeholders(v);
                new_map.insert(k, replaced_value);
            }
            Value::Mapping(new_map)
        }
        Value::Sequence(seq) => {
            let new_seq: Vec<Value> = seq.into_iter()
                .map(replace_placeholders)
                .collect();
            Value::Sequence(new_seq)
        }
        other => other,
    }
}
