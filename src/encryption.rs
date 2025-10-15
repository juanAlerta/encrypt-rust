use aes::Aes256;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::Key;
use cipher::generic_array::GenericArray;
use cipher::BlockEncrypt;

pub fn encrypt_aes(file_data: &[u8], key: &[u8; 32]) -> Vec<u8> {

    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut encrypted_data = Vec::new();

    for chunk in file_data.chunks(16) {
        let mut block = [0u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        let mut encrypted_block = block;

        cipher.encrypt_block(GenericArray::from_mut_slice(&mut encrypted_block));
        encrypted_data.extend_from_slice(&encrypted_block);
    }

    // Padding !
    if let Some(&padding_size) = encrypted_data.last(){
        let len = encrypted_data.len();
        if padding_size as usize <= len {
            encrypted_data.truncate(len - padding_size as usize);
        }
    }

    encrypted_data
}

pub fn encrypt_chacha(file_data: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let payload = Payload {
        msg: file_data,
        aad: b"optional_data",
    };
    cipher.encrypt(nonce.into(), payload).expect("Encryption failed")
}
