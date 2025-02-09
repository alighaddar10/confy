use std::process;
use serde_yaml::{Value, Mapping};
use crate::commands::parse::load_yaml_as_value;
use crate::commands::deepmerge::deep_merge;

/// build_final_config merges base.yaml + <env>.yaml
pub fn build_final_config(config_path: &str, env: &str) -> Value {
    let base_path = format!("{}/base.yaml", config_path);
    let env_path = format!("{}/{}.yaml", config_path, env);

    // Load base.yaml
    let base_val = match load_yaml_as_value(&base_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to load base.yaml from '{}': {}", base_path, e);
            process::exit(1);
        }
    };

    // Load env.yaml (if missing, treat it as empty)
    let env_val = load_yaml_as_value(&env_path).unwrap_or(Value::Mapping(Mapping::new()));

    // Deep merge
    deep_merge(base_val, env_val)
}

