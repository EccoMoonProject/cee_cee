# Cee Cee - CLI for secure storage of config files in repositories. The ultimate successor to the outdated .env




Cee Cee is a CLI that uses AES encryption standard to create encryption and decryption of config files (and any other type of files) in locations chosen by you, allowing you to safely store configuration files for your project in a repository. After pulling the project from the repository, you can use the CLI and the encryption key you generated prior to the process or received from your team-mates to decrypt the files. This provides a dynamic, secure way to store sensitive config files without the need for creating outdated and cumbersome .env files.

## Installation from source
mac os/linux users
```zsh
cargo build --release
```
add to bin
```zsh
sudo mv target/release/cee_cee /usr/local/bin
```
ensure
```zsh
sudo chmod +x /usr/local/bin/cee_cee
```

Windows:

Copy the binary to a directory in the system's PATH, such as C:\Windows\System32.

Alternatively, you can add the directory containing the binary to the PATH environment variable. To do this:

1. Right-click on 'Computer' or 'This PC' and select 'Properties'.
2. Click on 'Advanced system settings'.
3. Click on the 'Environment Variables' button.
4. Under 'System variables', find the 'Path' variable and click 'Edit'.
5. Add the directory containing your binary to the list, separated by a semicolon (;).
6. Click 'OK' to save your changes.
7. After completing these steps, users can run your CLI application using the desired command format.





## Usage

#### Init

```bash
  cee_cee init
```

It will run options menu to select your command

#### key generation

```bash
  cee_cee key <path_where_key_folder_will_be_stored>
```
#### key import 
```bash
cee_cee import_key <path_where_key_folder_will_be_stored> <base64 encoded key>
```

#### encryption 

```bash
  cee_cee encrypt <path_to_file> <path_to_key> <path_where_encrypted_data_folder_will_be_stored>
```
```bash
  cee_cee encrypt-ts <path_to_file> <path_to_key> <path_where_encrypted_data_folder_will_be_stored>
```

#### decryption

```bash
  cee_cee decrypt <path_to_key>  <path_to_file> <path_where_decrypted_data_folder_will_be_stored>
```
```bash
  cee_cee decrypt-ts <path_to_key>  <path_to_file> <path_where_decrypted_data_folder_will_be_stored>
```

## Authors

- [@mikolaj_rucinski](https://www.linkedin.com/in/mikolaj-rucinski)


## Tech Stack

**CLI** fully rust

