use serde_yaml::{Value, Mapping};

pub fn deep_merge(base: Value, env: Value) -> Value {
    match (base, env) {
        (Value::Mapping(mut base_map), Value::Mapping(env_map)) => {
            for (key, value) in env_map {
                base_map
                    .entry(key)
                    .and_modify(|existing| {
                        *existing = deep_merge(existing.clone(), value.clone());
                    })
                    .or_insert(value);
                }
            Value::Mapping(base_map)
        }
        (Value::Sequence(mut base_seq), Value::Sequence(env_seq)) => {
            base_seq.extend(env_seq);
            Value::Sequence(base_seq)
        }
        (_, env) => env,
    }
}
