use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub field1: String,
    pub field2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    pub key: String,
}
