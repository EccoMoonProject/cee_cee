#[path = "../core/crypto/crypto.rs"]
mod crypto;
#[path = "../core/parsing/parser.rs"]
mod parser;
#[path = "../core/reader/reader.rs"]
mod reader;
#[path = "../core/storage/storage.rs"]
mod storage;

#[path = "../core/structs/structs.rs"]
pub mod structs;
use colored::*;


pub fn decrypting_process(key_path: &str, file_to_decrypt: &str, decrypted_path: &str) {
    let mut key_to_encrypt: Vec<u8> = vec![0u8; 16];
    match reader::read_key_file_path_and_convert_to_u8(&key_path) {
        Ok(key) => key_to_encrypt = key,
        Err(e) => println!("Error: {}", e),
    }

    let mut to_be_decrypted = vec![0u8; 16];
    match reader::read_file_to_decrypt(&file_to_decrypt) {
        Ok(key) => to_be_decrypted = key,
        Err(e) => println!("Error: {}", e),
    }
    let decrypted = crypto::decrypt(&key_to_encrypt, &to_be_decrypted).unwrap();

    let string_decrypted = parser::u8vec_to_string(&decrypted);

    let val = parser::base64_to_value(&string_decrypted).unwrap();

    match storage::create_folder_and_write_decrypted_value(&val, &decrypted_path) {
        Ok(_) => println!("{}", "Successfully!!!".on_truecolor(135, 28, 167)),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn encrypting_process(file_path: &str, key_path: &str, encrypted_file_path: &str) {
    let mut key_to_encrypt: Vec<u8> = vec![0u8; 16];
    let data_to_encrypt = reader::read_file_as_value(file_path);

    let base64_encoded = parser::to_base64(&data_to_encrypt).unwrap();
    println!("{}","Data to encrypt: ".on_truecolor(115, 28, 147));
    println!("{}",base64_encoded);

    let parsed_to_u8 = parser::string_to_u8vec(&base64_encoded);

    match reader::read_key_file_path_and_convert_to_u8(key_path) {
        Ok(key) => key_to_encrypt = key,
        Err(e) => println!("Error: {}", e),
    }

    let encrypted = crypto::encrypt(&key_to_encrypt, &parsed_to_u8);

    match storage::create_folder_and_write_encrypted_struct(&encrypted, encrypted_file_path) {
        Ok(_) => println!(
            "{}",
            "Successfully written to the file".on_truecolor(115, 28, 147)
        ),
        Err(e) => println!("Error: {}", e),
    }
}


pub fn encrypting_process_ts(file_path: &str, key_path: &str, encrypted_file_path: &str) {
    let mut key_to_encrypt: Vec<u8> = vec![0u8; 16];
    let data_to_encrypt = reader::read_ts_js_file(file_path).unwrap();
    println!("{}","Data to encrypt: ".on_truecolor(115, 28, 147));
    println!("{}",data_to_encrypt);

    let parsed_to_u8 = parser::string_to_u8vec(&data_to_encrypt);

    match reader::read_key_file_path_and_convert_to_u8(key_path) {
        Ok(key) => key_to_encrypt = key,
        Err(e) => println!("Error: {}", e),
    }

    let encrypted = crypto::encrypt(&key_to_encrypt, &parsed_to_u8);

    match storage::write_string_to_file_ts(&encrypted, encrypted_file_path) {
        Ok(_) => println!(
            "{}",
            "Successfully written to the file".on_truecolor(115, 28, 147)
        ),
        Err(e) => println!("Error: {}", e),
    }
}


pub fn decrypting_process_ts(key_path: &str, encrypted_file_path: &str, decrypted_file_path: &str) {
    let mut key_to_decrypt: Vec<u8> = vec![0u8; 16];
    let encrypted_data_base64 = reader::read_ts_js_file(encrypted_file_path).unwrap();
    println!("{}","Data to decrypt: ".on_truecolor(115, 28, 147));
    println!("{}", encrypted_data_base64);

    let encrypted_data = base64::decode(&encrypted_data_base64).unwrap();

    match reader::read_key_file_path_and_convert_to_u8(key_path) {
        Ok(key) => key_to_decrypt = key,
        Err(e) => println!("Error: {}", e),
    }

    let decrypted_data = crypto::decrypt(&key_to_decrypt, &encrypted_data).unwrap();

    let decrypted_string = String::from_utf8(decrypted_data).unwrap();

    match storage::write_decrypted_data_to_ts_file(&decrypted_string, decrypted_file_path) {
        Ok(_) => println!(
            "{}",
            "Successfully written to the file".on_truecolor(115, 28, 147)
        ),
        Err(e) => println!("Error: {}", e),
    }
}
