use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use aes::cipher::generic_array::typenum::Unsigned;
use rand::Rng;

const BLOCK_SIZE: usize = 16;

pub fn generate_key() -> Vec<u8> {
    let mut key = vec![0u8; 16];
    rand::thread_rng().fill(&mut key[..]);
    key
}

fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded_data = data.to_vec();
    padded_data.extend(std::iter::repeat(padding_len as u8).take(padding_len));
    padded_data
}

fn pkcs7_unpad(padded_data: &[u8]) -> Option<Vec<u8>> {
    if let Some(&last_byte) = padded_data.last() {
        let padding_len = last_byte as usize;
        if padding_len > 0 && padding_len <= padded_data.len() {
            let data_len = padded_data.len() - padding_len;
            if padded_data[data_len..].iter().all(|&x| x == last_byte) {
                return Some(padded_data[..data_len].to_vec());
            }
        }
    }
    None
}

pub fn encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let padded_data = pkcs7_pad(data, BLOCK_SIZE);
    let num_blocks = padded_data.len() / BLOCK_SIZE;

    let mut encrypted = vec![0u8; num_blocks * BLOCK_SIZE];
    for i in 0..num_blocks {
        let block_start = i * BLOCK_SIZE;
        let block_end = block_start + BLOCK_SIZE;
        let mut block = GenericArray::clone_from_slice(&padded_data[block_start..block_end]);

        cipher.encrypt_block(&mut block);
        encrypted[block_start..block_end].copy_from_slice(&block[..]);
    }

    encrypted
}


pub fn decrypt(key: &[u8], encrypted: &[u8]) -> Option<Vec<u8>> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let data_len = encrypted.len();
    let num_blocks = data_len / BLOCK_SIZE;

    let mut decrypted = vec![0u8; data_len];
    for i in 0..num_blocks {
        let block_start = i * BLOCK_SIZE;
        let block_end = block_start + BLOCK_SIZE;
        let mut block = GenericArray::clone_from_slice(&encrypted[block_start..block_end]);

        cipher.decrypt_block(&mut block);
        decrypted[block_start..block_end].copy_from_slice(&block[..]);
    }

    pkcs7_unpad(&decrypted)
}
