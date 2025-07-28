use crate::models::credential::CredentialVault;
use crate::utils::crypto::{Encryptor, MasterKey, Passwordhasher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use base64;
use dirs;

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
        let vault_file = VaultFile {
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

    /// Load and decrypt vault with master password
    pub fn load_vault(&self, master_password: &str) -> anyhow::Result<CredentialVault> {
        let vault_file = self.load_vault_file()?;

        // verify master password
        if !Passwordhasher::verify_password(master_password, &vault_file.password_hash)? {
            return Err(anyhow::anyhow!("❌ Invalid master password"));
        }

        // decrypt vault data
        let salt_bytes = base64::decode(&vault_file.salt)
            .map_err(|e| anyhow::anyhow!("Salt decode error: {}", e))?;
        let master_key = MasterKey::derive_from_password(master_password, &salt_bytes)?;
        let decrypted_data = Encryptor::decrypt(&vault_file.encrypted_data, &master_key)?;

        // parse Json back to vault
        let vault_json = String::from_utf8(decrypted_data)?;
        let vault: CredentialVault = serde_json::from_str(&vault_json)?;

        println!("🔓 Vault unlocked Successfully!");
        Ok(vault)
    }

    /// Save encrypted vault to disk
    pub fn save_vault(&self, vault: &CredentialVault, master_password: &str) -> anyhow::Result<()> {
        let mut vault_file = self.load_vault_file()?;

        // verify master password
        if !Passwordhasher::verify_password(master_password, &vault_file.password_hash)? {
            return Err(anyhow::anyhow!("❌ Invalid master password"));
        }

        // Encrypt updated vault
        let salt_bytes = base64::decode(&vault_file.salt)
            .map_err(|e| anyhow::anyhow!("Salt decode Error: {}", e))?;
        let master_key = MasterKey::derive_from_password(master_password, &salt_bytes)?;
        let vault_json = serde_json::to_string(vault)?;
        let encrypted_data = Encryptor::encrypt(&master_key, vault_json.as_bytes())?;

        // Update vault file
        vault_file.encrypted_data = encrypted_data;
        vault_file.updated_at = chrono::Utc::now();

        self.save_vault_file(&vault_file)?;

        println!("💾 Vault saved successfullt!");

        Ok(())
    }

    /// Change master password (re-encrypt everything)
    pub fn change_master_password(
        &self,
        old_password: &str,
        new_password: &str,
    ) -> anyhow::Result<()> {
        // Load vault with old password
        let vault = self.load_vault(old_password)?;

        // Create new Password hash and Salt
        let (new_password_hash, new_salt_string) = Passwordhasher::hash_password(new_password)?;

        // Re-encrypt with new password
        let new_salt_bytes = base64::decode(&new_salt_string)
            .map_err(|e| anyhow::anyhow!("Salt decode error: {}", e))?;
        let new_master_key = MasterKey::derive_from_password(new_password, &new_salt_bytes)?;
        let vault_json = serde_json::to_string(&vault)?;
        let new_encrypted_data = Encryptor::encrypt(&new_master_key, vault_json.as_bytes())?;

        // create new vault file
        let vault_file = VaultFile {
            password_hash: new_password_hash,
            salt: new_salt_string,
            encrypted_data: new_encrypted_data,
            version: 1,
            updated_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        };

        self.save_vault_file(&vault_file)?;
        println!("🔑 Master password changed successfully!");

        Ok(())
    }

    /// Get vault path for display
    pub fn get_vault_path(&self) -> &Path {
        &self.vault_path
    }

    /// Internal: Load raw vault file from disk
    fn load_vault_file(&self) -> anyhow::Result<VaultFile> {
        if !self.vault_exists() {
            return Err(anyhow::anyhow!(
                "Vault not found at {}. Run 'init' to create one",
                self.vault_path.display()
            ));
        }

        let vault_data = fs::read(&self.vault_path)?;
        let vault_file = serde_json::from_slice(&vault_data)?;

        Ok(vault_file)
    }
    fn save_vault_file(&self, vault_file: &VaultFile) -> anyhow::Result<()> {
        let vault_json = serde_json::to_string_pretty(vault_file)?;

        // write to temporary file first (atomatic operation)
        let temp_path = self.vault_path.with_extension("tmp");
        fs::write(&temp_path, vault_json)?;

        // Set secure permission before moving
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&temp_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&temp_path, perms)?;
        }

        // Automically replace old file
        fs::rename(&temp_path, &self.vault_path)?;

        Ok(())
    }
}
