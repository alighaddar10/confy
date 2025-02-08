use std::process;
use serde_yaml::Value;
use similar::{ChangeTag, TextDiff};
use crate::commands::build_final_config::build_final_config;

pub fn handle_diff(env1: String, env2: String, config_path:String) {
    let final_val_1 = build_final_config(&config_path, &env1);
    let final_str_1 = serde_yaml::to_string(&final_val_1)
        .expect("Error in converting to string");
    
    let final_val_2 = build_final_config(&config_path, &env2);
    let final_str_2 = serde_yaml::to_string(&final_val_2)
        .expect("Error in converting to string");
    
    let diff = TextDiff::from_lines(&final_str_1, &final_str_2);

    println!("Comparing final config for '{}' vs '{}'", env1, env2);

    for group in diff.grouped_ops(3) {
        for op in group {
            for change in diff.iter_changes(&op) {
                match change.tag() {
                    ChangeTag::Delete => {
                        print!("-{}", change);
                    }
                    ChangeTag::Insert => {
                        print!("+{}", change);
                    }
                    ChangeTag::Equal => {
                        print!(" {}", change);
                    }
                }
            }
        }
    }
}
