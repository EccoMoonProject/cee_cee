use bincode::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::error::Error;

pub fn string_to_u8vec(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

pub fn u8vec_to_string(input: &[u8]) -> String {
    String::from_utf8_lossy(input).to_string()
}

pub fn value_to_u8(value: &Value) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = serde_json::to_vec(value)?;
    Ok(bytes)
}

pub fn u8_to_value(bytes: &[u8]) -> Result<Value, Box<dyn Error>> {
    let value = serde_json::from_slice(bytes)?;
    Ok(value)
}

pub fn to_base64(value: &Value) -> Result<String, Box<dyn Error>> {
    let bytes = serde_json::to_vec(value)?;
    let base64 = base64::encode(bytes);
    Ok(base64)
}

pub fn base64_to_value(base64: &str) -> Result<Value, Box<dyn Error>> {
    let bytes = base64::decode(base64)?;
    let value = serde_json::from_slice(&bytes)?;
    Ok(value)
}

pub fn vec_u8_to_string(data: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(data)
}


pub fn u8vec_to_hex(input: &[u8]) -> String {
    let mut hex = String::new();
    for byte in input {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}