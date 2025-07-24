use crate::models::credential::CredentialVault;
use crate::utils::crypto::{Encryption, MasterKey, Passwordhasher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const VAULT_FILE_EXTENSION: &str = "vault";

//  Metadata and encryption data stored in vault file
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultFile {
    pub password_hash: String,   // Argon2id hash for authencation
    pub salt: String,            // Salt for key derivation (base64)
    pub encrypted_data: Vec<u8>, // AES-encrypted credential data
    pub version: u32,            // File formate version
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
/// Headles Secure file operation for password vault
pub struct FileStorage {
    vault_path: PathBuf,
}

/// Feature to build
// => Encrypted file storage - save vault data securely to disk
impl FileStorage {}
// => Master Password Auth - Verify user before decryption
// => Cross-Platform Paths - Works on windows, macOs, Linux
// => Atomatic Operations - Prevent data corruption
// => Proper File Permissions - Secure file access (Unix)
