use jsonschema::{JSONSchema, Draft};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::error::Error;
use std::fs;

pub fn validate_with_schema(merged_val: &YamlValue, schema_path: &str) -> Result<(), Box<dyn Error>> {
    // 1. Convert the final YAML to JSON
    let merged_json: JsonValue = serde_json::to_value(merged_val)?;

    // 2. Load and parse the JSON schema from a file
    let schema_str = fs::read_to_string(schema_path)?;
    let schema_json: JsonValue = serde_json::from_str(&schema_str)?;

    // 3. Compile the schema
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json)
        .map_err(|e| format!("Invalid schema: {}", e))?;

    // 4. Validate the merged JSON
    if let Err(errors) = compiled_schema.validate(&merged_json) {
        let msgs = errors.map(|e| e.to_string()).collect::<Vec<String>>();
        // Return an error listing all validation messages
        return Err(format!("Schema validation errors:\n{}", msgs.join("\n")).into());
    }

    Ok(())
}
