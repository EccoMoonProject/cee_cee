use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json::Value;
#[path = "../core/crypto/crypto.rs"]
mod crypto;
#[path = "../cli/functions.rs"]
mod functions;
#[path = "../core/parsing/parser.rs"]
mod parser;
#[path = "../core/reader/reader.rs"]
mod reader;
#[path = "../core/storage/storage.rs"]
mod storage;
#[path = "../core/structs/structs.rs"]
pub mod structs;
use base64::{decode, encode};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use functions::*;
use serde::{Deserialize, Serialize};

pub fn cli_command_line(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("encrypt") {
        let ascii = r#"
        ___       _  __      __ ___ ___       _        
        )_  )\ ) / ` )_) \_) )_) )   )  )\ ) / _       
       (__ (  ( (_. / \   / /   (  _(_ (  ( (__/ o o o 
                                                       
"#;
        println!("{}", ascii.truecolor(115, 28, 147));
        let file_path = matches.value_of("input").unwrap();
        let key_path = matches.value_of("key_path").unwrap();
        let encrypted_file_path = matches.value_of("encrypt_path").unwrap();

        encrypting_process(file_path, key_path, encrypted_file_path);
    } else if let Some(matches) = matches.subcommand_matches("encrypt-ts") {
        let ascii = r#"
        ___       _  __      __ ___ ___       _        
        )_  )\ ) / ` )_) \_) )_) )   )  )\ ) / _       
       (__ (  ( (_. / \   / /   (  _(_ (  ( (__/ o o o 
                                                       
"#;
        println!("{}", ascii.truecolor(115, 28, 147));

        let file_path = matches.value_of("input").unwrap();
        let key_path = matches.value_of("key_path").unwrap();
        let encrypted_file_path = matches.value_of("encrypt_path").unwrap();

        encrypting_process_ts(file_path, key_path, encrypted_file_path);
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let ascii = r#"

 __  ___  _  __      __ ___ ___       _        
) ) )_  / ` )_) \_) )_) )   )  )\ ) / _       
/_/ (__ (_. / \   / /   (  _(_ (  ( (__/ o o o 
                                              
"#;
        println!("{}", ascii.truecolor(115, 28, 147));
        let key_path = matches.value_of("key").unwrap();
        let file_to_decrypt = matches.value_of("file_to_decrypt").unwrap();
        let decrypted_path = matches.value_of("decrypted_path").unwrap();
        decrypting_process(key_path, file_to_decrypt, decrypted_path);
    } else if let Some(matches) = matches.subcommand_matches("decrypt-ts") {
        let ascii = r#"

 __  ___  _  __      __ ___ ___       _        
) ) )_  / ` )_) \_) )_) )   )  )\ ) / _       
/_/ (__ (_. / \   / /   (  _(_ (  ( (__/ o o o 
                                              
"#;
        println!("{}", ascii.truecolor(115, 28, 147));
        let key_path = matches.value_of("key").unwrap();
        let file_to_decrypt = matches.value_of("file_to_decrypt").unwrap();
        let decrypted_path = matches.value_of("decrypted_path").unwrap();
        decrypting_process_ts(key_path, file_to_decrypt, decrypted_path);
    } else if let Some(matches) = matches.subcommand_matches("key") {
        let ascii = r#"

  _   ___      ___ __   _ ___ ___       _         ___         
  / _  )_  )\ ) )_  )_) /_) )   )  )\ ) / _   )_/  )_ \_)      
 (__/ (__ (  ( (__ / \ / / (  _(_ (  ( (__/  /  ) (__  / o o o 
                                                               
"#;
        println!("{}", ascii.truecolor(135, 28, 167));
        let key_path = matches.value_of("key_path").unwrap();
        let key = crypto::generate_key();

        match storage::create_folder_and_write_key(&key, &key_path) {
            Ok(_) => println!(
                "{}",
                "Key successfully written to the file.".on_truecolor(135, 28, 167)
            ),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("import_key") {
        let key_path = matches.value_of("key_path").unwrap();
        let key = decode(matches.value_of("key").unwrap()).unwrap();

        match storage::create_folder_and_write_key(&key, &key_path) {
            Ok(_) => println!(
                "{}",
                "Key successfully written to the file.".on_truecolor(135, 28, 167)
            ),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("init") {
        let ascii = r#"
        ___          ___          ___          ___     
       /\  \        /\  \        /\  \        /\  \    
      /::\  \      /::\  \      /::\  \      /::\  \   
     /:/\:\  \    /:/\:\  \    /:/\:\  \    /:/\:\  \  
    /:/  \:\  \  /::\~\:\  \  /:/  \:\  \  /::\~\:\  \ 
   /:/__/ \:\__\/:/\:\ \:\__\/:/__/ \:\__\/:/\:\ \:\__\
   \:\  \  \/__/\:\~\:\ \/__/\:\  \  \/__/\:\~\:\ \/__/
    \:\  \       \:\ \:\__\   \:\  \       \:\ \:\__\  
     \:\  \       \:\ \/__/    \:\  \       \:\ \/__/  
      \:\__\       \:\__\       \:\__\       \:\__\    
       \/__/        \/__/        \/__/        \/__/    
  "#;
        println!("{}", ascii.truecolor(115, 28, 147));
        print!("\n");
        let commands = &[
            "encrypt",
            "encrypt-ts",
            "decrypt-ts",
            "decrypt",
            "key",
            "import_key",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a command")
            .default(0)
            .items(commands)
            .interact()
            .unwrap();

        println!("Selected command: {}", commands[selection]);

        // Continue with the chosen command and ask for the required arguments
        match commands[selection] {
            "encrypt" => {
                let input_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the JSON file to encrypt")
                    .interact_text()
                    .unwrap();
                let key_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the key to encrypt")
                    .interact_text()
                    .unwrap();
                let encrypt_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to folder where encrypted JSON will be stored")
                    .interact_text()
                    .unwrap();

                encrypting_process(&input_path, &key_path, &encrypt_path);
            }
            "encrypt-ts" => {
                let input_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the JSON file to encrypt")
                    .interact_text()
                    .unwrap();
                let key_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the key to encrypt")
                    .interact_text()
                    .unwrap();
                let encrypt_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to folder where encrypted JSON will be stored")
                    .interact_text()
                    .unwrap();

                encrypting_process_ts(&input_path, &key_path, &encrypt_path);
            }
            "decrypt" => {
                let key_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the JSON file containing the AES key")
                    .interact_text()
                    .unwrap();
                let file_to_decrypt: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the JSON file")
                    .interact_text()
                    .unwrap();
                let decrypted_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to folder where decrypted JSON will be stored")
                    .interact_text()
                    .unwrap();

                decrypting_process(&key_path, &file_to_decrypt, &decrypted_path);
            }
            "decrypt-ts" => {
                let key_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the JSON file containing the AES key")
                    .interact_text()
                    .unwrap();
                let file_to_decrypt: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to the JSON file")
                    .interact_text()
                    .unwrap();
                let decrypted_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to folder where decrypted JSON will be stored")
                    .interact_text()
                    .unwrap();

                decrypting_process_ts(&key_path, &file_to_decrypt, &decrypted_path);
            }
            "key" => {
                let key_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to folder where the key will be stored")
                    .interact_text()
                    .unwrap();

                let key = crypto::generate_key();

                match storage::create_folder_and_write_key(&key, &key_path) {
                    Ok(_) => println!(
                        "{}",
                        "Key successfully written to the file.".on_truecolor(135, 28, 167)
                    ),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "import_key" => {
                let key_path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the path to folder where the key will be stored")
                    .interact_text()
                    .unwrap();
                let key: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the key to be imported")
                    .interact_text()
                    .unwrap();

                let key_decoded = decode(&key).unwrap();

                match storage::create_folder_and_write_key(&key_decoded, &key_path) {
                    Ok(_) => println!(
                        "{}",
                        "Key successfully written to the file.".on_truecolor(135, 28, 167)
                    ),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            _ => {
                eprintln!("Invalid command selected");
            }
        }
    } else {
        eprintln!(
            "{}",
            "No command provided. Use --help for more information.".on_truecolor(212, 28, 13)
        );
    }
}
pub(crate) fn create_cli_app() -> ArgMatches {
    let matches = App::new("CeeCee CLI")
        .version("0.1")
        .author("Mikolaj Rucinski rucinski46@icloud.com")
        .about("CLI for secure storage of config files in repositories. The ultimate successor to the outdated .env")
        .subcommand(
            SubCommand::with_name("encrypt")
                .about("Encrypts a JSON file using a generated AES key")
                .arg(
                    Arg::with_name("input")
                        .help("Path to the JSON file to encrypt")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("key_path")
                        .help("Path to the key to encrypt")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("encrypt_path")
                        .help("Path to folder where encrypted JSON will be stored")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            SubCommand::with_name("encrypt-ts")
                .about("Encrypts a JSON file using a generated AES key")
                .arg(
                    Arg::with_name("input")
                        .help("Path to the JSON file to encrypt")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("key_path")
                        .help("Path to the key to encrypt")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("encrypt_path")
                        .help("Path to folder where encrypted JSON will be stored")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("Decrypts encrypted data using a provided AES key")
                .arg(
                    Arg::with_name("key")
                        .help("Path to the JSON file containing the AES key")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("file_to_decrypt")
                        .help("Path to the JSON file")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("decrypted_path")
                        .help("Path to folder where decrypted JSON will be stored")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt-ts")
                .about("Decrypts encrypted data using a provided AES key")
                .arg(
                    Arg::with_name("key")
                        .help("Path to the JSON file containing the AES key")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("file_to_decrypt")
                        .help("Path to the JSON file")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("decrypted_path")
                        .help("Path to folder where decrypted JSON will be stored")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            SubCommand::with_name("key").about("Generate key").arg(
                Arg::with_name("key_path")
                    .help("path to folder where the key will be stored")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("import_key")
                .about("Import key for symmetric encryption")
                .arg(
                    Arg::with_name("key_path")
                        .help("path to folder where the key will be stored")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("key")
                        .help("key to be imported")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize the cece_cli")
        )
        .get_matches();

    matches
}
