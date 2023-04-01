#[path = "../structs/structs.rs"]
pub mod structs;
use crate::methods::structs::Config; // Change this line
use serde_json::Value;
use serde_json::json;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn create_folder_and_write_key(
    key: &[u8],
    path_input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_base64 = base64::encode(&key);

    let base_path = Path::new(&path_input);
    let folder_path = base_path.join("aes_key_folder");

    // Create folder
    fs::create_dir_all(&folder_path)?;

    // Create JSON file with the AES key
    let file_path = folder_path.join("key.json");
    let mut file = File::create(&file_path)?;

    // Write key object to the file
    let key_object = json!({ "key": key_base64 });
    file.write_all(key_object.to_string().as_bytes())?;

    Ok(())
}

pub fn create_folder_and_write_encrypted_struct(
    encrypted_struct: &[u8],
    path_input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&path_input);
    let folder_path = base_path.join("encrypted_struct_folder");

    // Create folder
    fs::create_dir_all(&folder_path)?;

    // Create JSON file with the AES key
    let file_path = folder_path.join("encrypted_struct.json");
    let mut file = File::create(&file_path)?;

    // Write key object to the file
    let encrypted_struct_object = json!({ "encrypted_struct": encrypted_struct });
    file.write_all(encrypted_struct_object.to_string().as_bytes())?;

    Ok(())
}


pub fn create_folder_and_write_decrypted_value(
    decrypted_value: &Value,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&path);
    let folder_path = base_path.join("decrypted_value_folder");

    // Create folder
    fs::create_dir_all(&folder_path)?;

    // Create JSON file with the decrypted value
    let file_path = folder_path.join("config.json");
    let mut file = std::fs::File::create(&file_path)?;

    file.write_all(decrypted_value.to_string().as_bytes())?;

    Ok(())
}

pub fn write_string_to_file_ts(data: &[u8], output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a directory if it does not exist
    let output_file_path = Path::new(output_path);
    if let Some(parent_dir) = output_file_path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }

    // Write the input string to the output file
    let mut file = File::create(output_file_path)?;
    file.write_all(data)?;

    Ok(())
}
pub fn write_decrypted_data_to_ts_file(decrypted_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // decode the base64 string
    let decrypted_data = base64::decode(decrypted_data)?;
    let output_file_path = Path::new(output_path);
    if let Some(parent_dir) = output_file_path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }

    let mut file = File::create(output_file_path)?;
    file.write_all(&decrypted_data)?;

    Ok(())
}