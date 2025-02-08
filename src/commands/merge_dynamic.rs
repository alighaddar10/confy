fn merge_dynamic_configs(base_path: &str, env_path: &str) -> Result<Value, Box<dyn std:error:Error>> {
    let base_val = load_yaml_as_value(base_path)?;
    let env_val = load_yaml_as_value(env_path)?;
    let merged = deep_merge(base_val, env_val);
    Ok(merged)
}
