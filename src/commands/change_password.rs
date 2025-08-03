use crate::commands::{ensure_vault_exists, get_master_password, get_storage};
use anyhow::Result;

pub fn run() -> Result<()> {
    let storage = get_storage()?;
    ensure_vault_exists(&storage)?;

    // Prompt for the current master password
    let old_password = get_master_password("Enter current master password:")?;

    // Prompt for the new master passowrd and confirmation
    let new_password = get_master_password("Enter new master password:")?;
    let confirm_password = get_master_password("Confirm new master password:")?;

    // Validate new password confirmation
    if new_password != confirm_password {
        return Err(anyhow::anyhow!("New passwords do not match!"));
    }

    // Enforce password lenght policy
    if new_password.len() < 8 {
        return Err(anyhow::anyhow!(
            "New Passowrd must be at least 8 characters long."
        ));
    }

    // Attempt to change the master password securely (re-encrypt vault)
    storage.change_master_password(&old_password, &new_password)?;

    println!("🔑 Master password changed successfully!");

    Ok(())
}
