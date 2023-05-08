//! A CLI tool that reads a JSON file and creates a text output of a Rust structs or
//! TypeScript interfaces that represent the objects from the JSON file.
//! It takes in a file path and a flag to specify whether to print out a TypeScript
//! interface or Rust struct.

use serde_json::Value;
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <file_path> <rust|typescript>", args[0]);
        return;
    }

    let file_path = &args[1];
    let format = &args[2];

    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let json_value: Value = serde_json::from_str(&content).expect("Failed to parse JSON");

    if !json_value.is_array() {
        println!("The input JSON file should contain an array of objects.");
        return;
    }

    let json_array = json_value.as_array().unwrap();

    let output = match format.as_str() {
        "rust" => {
            let struct_fields = print_rust_struct(json_array);
            format!("struct Data {{\n{}}}", struct_fields)
        }
        "typescript" => {
            let interface_fields = print_typescript_interface(json_array);
            format!("interface Data {{\n{}}}", interface_fields)
        }
        _ => {
            println!("Invalid format. Please use 'rust' or 'typescript'.");
            return;
        }
    };

    println!("{}", output);
}

fn rust_value_type(value: &Value) -> String {
    match value {
        Value::String(_) => "String".to_string(),
        Value::Number(n) if n.is_i64() => "i64".to_string(),
        Value::Number(n) if n.is_u64() => "u64".to_string(),
        Value::Number(_) => "f64".to_string(),
        Value::Bool(_) => "bool".to_string(),
        Value::Array(_) => "Vec<Value>".to_string(),
        Value::Object(_) => "HashMap<String, Value>".to_string(),
        Value::Null => "Value".to_string(),
    }
}

fn print_rust_struct(values: &[Value]) -> String {
    let mut output = String::new();
    let mut fields: BTreeMap<String, (bool, String)> = BTreeMap::new();

    for value in values {
        if let Value::Object(map) = value {
            for (key, value) in map {
                fields
                    .entry(key.clone())
                    .and_modify(|(is_optional, ty)| {
                        *is_optional = *is_optional && value.is_null();
                        if value.is_null() {
                            return;
                        }
                        let new_ty = rust_value_type(value);
                        if ty != &new_ty && !value.is_null() {
                            *ty = new_ty;
                        }
                    })
                    .or_insert((true, rust_value_type(value)));
            }
        }
    }

    for (key, (is_optional, value)) in &fields {
        output.push_str(&format!("    {}: ", key));
        output.push_str(if *is_optional { "Option<" } else { "" });
        output.push_str(value);
        output.push_str(if *is_optional { ">" } else { "" });
        output.push_str(",\n");
    }
    output
}

fn print_typescript_interface(values: &[Value]) -> String {
    let mut output = String::new();
    let mut fields: BTreeMap<String, (bool, String)> = BTreeMap::new();

    for value in values {
        if let Value::Object(map) = value {
            for (key, value) in map {
                fields
                    .entry(key.clone())
                    .and_modify(|(is_optional, ty)| {
                        *is_optional = *is_optional && value.is_null();
                        if value.is_null() {
                            return;
                        }
                        let new_ty = typescript_value_type(value);
                        if ty != &new_ty && !value.is_null() {
                            *ty = new_ty.to_owned();
                        }
                    })
                    .or_insert((true, typescript_value_type(value).to_owned()));
            }
        }
    }

    for (key, (is_optional, ty)) in &fields {
        output.push_str(&format!(
            "    {}{}: ",
            key,
            if *is_optional { "?" } else { "" }
        ));
        output.push_str(ty);
        output.push_str(";\n");
    }
    output
}

fn typescript_value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "Array",
        Value::Object(_) => "Record<string, unknown>",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn format_output(s: &str) -> String {
        s.lines()
            .map(|line| {
                let line = line.trim_start();
                if line.ends_with(',') {
                    let len = line.len();
                    format!("{};", &line[0..len - 1])
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn test_print_rust_struct() {
        let json_value = json!([
            {
                "name": "Alice",
                "age": 30.0,
                "is_student": false
            },
            {
                "name": "Bob",
                "age": 25.0,
                "is_student": true,
                "address": null
            }
        ]);

        let expected_output =
            "address: Option<Value>;\nage: f64;\nis_student: bool;\nname: String;";

        let actual_output = format_output(&print_rust_struct(&json_value.as_array().unwrap()));
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_print_rust_struct_partial_fields() {
        let json_value = json!([
            {
                "name": "Alice",
                "age": 30.0
            },
            {
                "name": "Bob",
                "is_student": true
            }
        ]);

        let expected_output = "age: Option<f64>;\nis_student: Option<bool>;\nname: String;";

        let actual_output = format_output(&print_rust_struct(&json_value.as_array().unwrap()));
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_print_typescript_interface() {
        let json_value = json!([
            {
                "name": "Alice",
                "age": 30.0,
                "is_student": false
            },
            {
                "name": "Bob",
                "age": 25.0,
                "is_student": true
            }
        ]);

        let expected_output = "age: number;\nis_student: boolean;\nname: string;";

        let actual_output =
            format_output(&print_typescript_interface(&json_value.as_array().unwrap()));
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_print_typescript_interface_partial_fields() {
        let json_value = json!([
            {
                "name": "Alice",
            },
            {
                "name": "Bob",
                "is_student": true,
            }
        ]);

        let expected_output = "is_student?: boolean;\nname: string;";

        let actual_output =
            format_output(&print_typescript_interface(&json_value.as_array().unwrap()));
        assert_eq!(actual_output, expected_output);
    }
}
