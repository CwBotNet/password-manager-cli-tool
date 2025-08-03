use crate::commands::{ensure_vault_exists, get_master_password, get_storage};
use anyhow::Result;

pub fn run() -> Result<()> {
    let storage = get_storage()?;
    ensure_vault_exists(&storage)?;

    let master_password = get_master_password("🔐 Enter master password to unlock vault:")?;
    let vault = storage.load_vault(&master_password)?;

    println!("📊 Vault Statistics:");
    println!("  . Total credentials: {}", vault.credentials.len());
    println!("  . Vault version: {}", vault.version);
    println!(
        "  . Last updated: {}",
        vault.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "  . Created on: {}",
        vault.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    Ok(())
}
