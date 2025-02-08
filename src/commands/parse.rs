use serde_yaml::Value;
use std::fs;

pub fn load_yaml_as_value(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let value: Value = serde_yaml::from_str(&content)?;
    Ok(value)
}
