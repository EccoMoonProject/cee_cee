use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
#[path = "../parsing/parser.rs"]
pub mod parser;
#[path = "../structs/structs.rs"]
pub mod structs;
use crate::methods::structs::Config; // Change this line

pub fn read_file_as_value(input: &str) -> Value {
    // Read JSON file
    let path = PathBuf::from(input);
    let mut file = File::open(&path).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    // Deserialize JSON
    let json_value: Value = serde_json::from_str(&contents).expect("Could not deserialize JSON");

    json_value
}

pub fn read_key_file_path_and_convert_to_u8(
    file_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the JSON object
    let json_value: Value = serde_json::from_str(&contents)?;
    let key_base64 = json_value["key"]
        .as_str()
        .ok_or("Failed to read key from JSON")?;

    // Convert the base64-encoded key to a Vec<u8>
    let key = base64::decode(&key_base64)?;

    Ok(key)
}

pub fn read_file_to_decrypt(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the JSON object
    let json_value: Value = serde_json::from_str(&contents)?;

    // Get the encrypted_struct value as a Vec<u8>
    let encrypted_struct = json_value["encrypted_struct"]
        .as_array()
        .ok_or("Missing encrypted_struct field or not an array")?
        .iter()
        .map(|v| {
            v.as_u64()
                .ok_or("Array element is not a valid u64 value")
                .and_then(|val| {
                    u8::try_from(val).map_err(|_| "Array element is out of range for u8")
                })
        })
        .collect::<Result<Vec<u8>, _>>()?;
    Ok(encrypted_struct)
}

