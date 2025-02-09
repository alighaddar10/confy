use std::{fs, process, io::Write};
use crate::commands::build_final_config::build_final_config;
use crate::commands::replace_placeholders::replace_placeholders;
use crate::commands::validate_with_schema::validate_with_schema;

 pub fn handle_build(env: String, output: Option<String>, config_path: String, validate: Option<bool>) {
    // 1. Build final merged config
    let merged_val = build_final_config(&config_path, &env);

    // 2. Replace placeholders
    let replaced_val = replace_placeholders(merged_val);

    // 3. Validate against a JSON schema
    if let Some(true) = validate {
        let schema_file = format!("{}/schema.json", config_path);
        if let Err(e) = validate_with_schema(&replaced_val, &schema_file) {
            eprintln!("Configuration is invalid:\n{}", e);
            std::process::exit(1);
        }
    }

    // 3. Serialize to YAML
    let merged_yaml = match serde_yaml::to_string(&replaced_val) {
        Ok(yaml) => yaml,
        Err(e) => {
            eprintln!("Failed to serialize merged YAML: {}", e);
            process::exit(1);
        }
    };

    // 4. Output to file or stdout
    if let Some(out_path) = output {
        let mut file = match fs::File::create(&out_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to create output file {}: {}", out_path, e);
                process::exit(1);
            }
        };
        if let Err(e) = file.write_all(merged_yaml.as_bytes()) {
            eprintln!("Failed to write to output file {}: {}", out_path, e);
            process::exit(1);
        }
    } else {
        println!("{}", merged_yaml);
    }
}
