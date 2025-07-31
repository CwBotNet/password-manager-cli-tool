pub mod add;
pub mod change_password;
pub mod delete;
pub mod get;
pub mod init;
pub mod list;
pub mod search;
pub mod status;

use crate::storage::file::FileStorage;
use anyhow::Result;
use rpassword;

/// Common utility: Get master password from user securely
pub fn get_master_password(prompt: &str) -> Result<String> {
    println!("{}", prompt);
    let password = rpassword::read_password()?;

    if password.trim().is_empty(){
        return Err(anyhow::anyhow!("❌ Password cannot be empty"));
    }

    Ok(password)
}

/// Common utility: Get Storage instance
pub fn get_storage()-> Result<FileStorage>{
    FileStorage::new()
}

/// Common utility: Check if Vault exists and guide user
pub fn ensure_vault_exists(storage: &FileStorage) ->Result<()>{
    if !storage.vault_exists(){
        println!("❌ No vault found.");
        println!("💡 Initialize a new vault with: pwdmgr init");
        return Err(anyhow::anyhow!("Vault not found"));
    }

    Ok(())
}