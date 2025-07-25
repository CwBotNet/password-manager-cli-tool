use crate::models::credential::CredentialVault;
use crate::utils::crypto::{Encryptor, MasterKey, Passwordhasher};
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
impl FileStorage {
    /// Initialize storage in user's home directory
    pub fn new() -> anyhow::Result<Self> {
        let vault_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join(".password_manager");

        // Create Secure directory
        if !vault_dir.exists() {
            fs::create_dir_all(&vault_dir)?;

            // Set restrictive permission on Unix systems

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&vault_dir)?.permissions();
                perms.set_mode(0o700); // rwx------ (Owner only)
                fs::set_permissions(&vault_dir, perms)?;
            }

            println!("🔐 Created Secure vault directory: {}", vault_dir.display());
        }
        let vault_path = vault_dir.join(format!("vault.{}", VAULT_FILE_EXTENSION));

        Ok(Self { vault_path })
    }

    /// check if vault file exists
    pub fn vault_exists(&self) -> bool {
        self.vault_path.exists()
    }

    /// Initialize new vault with master password
    pub fn initialize_vault(&self, master_password: &str) -> anyhow::Result<()> {
        if self.vault_exists() {
            return Err(anyhow::anyhow!(
                "Vault already existes at {}",
                self.vault_path.display()
            ));
        }

        // Create password hash for authencation
        let (password_hash, salt_string) = Passwordhasher::hash_password(master_password)?;

        // Create empty vault
        let empty_vault = CredentialVault::new();
        let vault_json = serde_json::to_string(&empty_vault)?;

        // Encrypt vault data
        let salt_byte = base64::decode(&salt_string)
            .map_err(|e| anyhow::anyhow!("salt decode Error: {}", e))?;

        let master_key = MasterKey::derive_from_password(master_password, &salt_byte)?;
        let encrypted_data = Encryptor::encrypt(&master_key, vault_json.as_bytes())?;

        // Create Vault file
        let vault_file = VaultFile{
            password_hash,
            salt: salt_string,
            encrypted_data,
            version: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.save_vault_file(&vault_file)?;

        println!("✅ Vault initialized successfully!");
        println!("📍 Location: {}", self.vault_path.display());

        Ok(())

    }
    fn save_vault_file(&self, vault_file: &VaultFile) -> anyhow::Result<()>{
        let vault_json = serde_json::to_string_pretty(vault_file)?;
    
        // write to temporary file first (atomatic operation)
        let temp_path = self.vault_path.with_extension("tmp");
        fs::write(&temp_path, vault_json)?;

        // Set secure permission before moving
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&temp_path)?.permissions();
            fs::set_permissions(&temp_path, perms)?;
        }

        // Automically replace old file
        fs::rename(&temp_path, &self.vault_path)?;

        Ok(())
    }
}
// => Master Password Auth - Verify user before decryption
// => Cross-Platform Paths - Works on windows, macOs, Linux
// => Atomatic Operations - Prevent data corruption
// => Proper File Permissions - Secure file access (Unix)
