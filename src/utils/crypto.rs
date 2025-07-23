// Encryption & decryption helper
use aes_gcm::{
    AeadCore, Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use anyhow::{self, Ok};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash};
use zeroize::Zeroize;

/// Constants
const SALT_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

/// Secured wrapper over the derived encryption key
pub struct MasterKey {
    key: [u8; 32],
}

impl MasterKey {
    /// Derive encryption key from master password using argon2id KDF
    pub fn derive_from_password(password: &str, salt: &[u8]) -> anyhow::Result<Self> {
        if salt.len() != SALT_LENGTH {
            return Err(anyhow::anyhow!("Invalid salt length"));
        }

        let argon2 = Argon2::default();
        let mut key = [0u8; 32]; // 256-bit key

        // Fill key with password-derived bytes
        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| anyhow::anyhow!("KDF failed:{}", e))?;
        Ok(Self { key })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }
}

// this impl make sure we wipe sensitive key meterial form memory
impl Drop for MasterKey {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

impl Zeroize for MasterKey {
    fn zeroize(&mut self) {
        self.key.zeroize();
    }
}

pub struct Encryption;

impl Encryption {
    /// Encrypt given plantext using AES-256-GCM
    pub fn encrypt_password(
        master_key: &MasterKey,
        associated_data: &[u8],
    ) -> anyhow::Result<Vec<u8>> {
        let key = Key::<Aes256Gcm>::from_slice(master_key.as_bytes());
        let cipher = Aes256Gcm::new(key);

        // genrate a random nonce for each time enctryption
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, associated_data)
            .map_err(|e| anyhow::anyhow!("Encryption error: {}", e))?;

        // Concatenate nonce + ciphertext
        let mut result = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }
    /// Decrypt given data using AES-256-GCM
    pub fn decrypt_password(encrypted: &[u8], master_key: &MasterKey) -> anyhow::Result<Vec<u8>> {
        if encrypted.len() < NONCE_LENGTH {
            return Err(anyhow::anyhow!("Encrypted data too short"));
        }

        let (nonce_byte, ciphertext) = encrypted.split_at(NONCE_LENGTH);
        let nonce = Nonce::from_slice(nonce_byte);
        let key = Key::<Aes256Gcm>::from_slice(master_key.as_bytes());
        let cipher = Aes256Gcm::new(key);

        let plane_text = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption error: {}", e))?;

        Ok(plane_text)
    }
}

pub struct Passwordhasher;

impl Passwordhasher {
    /// create hash of master password (for authentication)
    pub fn hash_password(password: &str) -> anyhow::Result<(String, String)> {
        let salt = password_hash::SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Hashing failed:  {}", e))?
            .to_string();

        Ok((hash, salt.to_string()))
    }

    /// verify given password against stoored hash
    pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
        let password_hash =
            PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;

        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok())
    }
}
