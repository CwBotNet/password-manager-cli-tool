// Encryption & decryption helper
use aes_gcm::{
    AeadCore, Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, Payload},
};

pub fn encrypt_password(
    password: &[u8],
    key: &[u8; 32],
    associated_data: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), aes_gcm::Error> {
    let cipher = Aes256Gcm::new(key.into());

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher_text = cipher.encrypt(
        &nonce,
        Payload {
            msg: password,
            aad: associated_data,
        },
    )?;

    Ok((cipher_text, nonce.to_vec()))
}

pub fn decrypt_password(
    ciphertext: &[u8],
    key: &[u8; 32],
    nonce_bytes: &[u8],
    associated_data: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes256Gcm::new(key.into());

    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce,Payload{
        msg:ciphertext,
        aad:associated_data
    })?;

    println!(
        "{:?} {:?} {:?} {:?}",
        ciphertext, key, nonce_bytes, associated_data
    );
    Ok(plaintext)
}
