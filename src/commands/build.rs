use std::{fs, process, io::Write};
use clap::Parser;
use serde_yaml::Value;
use crate::commands::build_final_config::build_final_config;

 pub fn handle_build(env: String, output: Option<String>, config_path: String) {
    // 1. Build final merged config
    let merged = build_final_config(&config_path, &env);

    // 2. Serialize to YAML
    let merged_yaml = match serde_yaml::to_string(&merged) {
        Ok(yaml) => yaml,
        Err(e) => {
            eprintln!("Failed to serialize merged YAML: {}", e);
            process::exit(1);
        }
    };

    // 3. Output to file or stdout
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
