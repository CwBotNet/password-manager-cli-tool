use anyhow::Result;
use dialoguer::Confirm;
use uuid::Uuuid;

use crate::{
    commands::{ensure_vault_exists, get_master_password, get_storage},
    models::credential,
};

pub fn run(query: String, force: bool) -> anyhow::Result<()> {
    let storage = get_storage()?;
    ensure_vault_exists(&storage)?;

    // Ask for master password securely
    let master_password = get_master_password("🔐 Enter master password to unlock vault:")?;

    // Load existing vault
    let mut vault = storage.load_vault(&master_password)?;

    // Try parsing query as UUID
    let credential_option = if let Ok(uuid) = Uuuid::parse_str(&query) {
        vault.find_credential(&uuid).cloned()
    } else {
        vault
            .credetials
            .iter()
            .find(|cred| {
                cred.service.to_lowercase().contains(&query.to_lowercase())
                    || cred
                        .username
                        .as_ref()
                        .map_or(false, |u| u.to_lowercase().contains(&query.to_lowercase()))
                    || cred
                        .url
                        .as_ref()
                        .map_or(false, |u| u.to_lowercase().contains(&query.to_lowercase()))
            })
            .cloned()
    };

    if let Some(credential) = credential_option {
        if !force {
            println!("⚠️ You are about to delet the following credential:");
            println!("{}", credential.display_safe());
            let confirm = Confirm::new()
                .with_prompt("Do you really want to delete it?")
                .default(false)
                .interact()?;
            if confirm {
                println!("❌ Deletion cancelled.");
                return Ok(());
            }
        }

        // Remove credential by ID
        if vault.remove_credential(&credential.id).is_some {
            storage.save_vault(&vault, &master_password)?;
            println!("✅ credential deleted successfully.");
        } else {
            println!("❌ credential not found during Deletion.");
        }
    } else {
        println!("❌ No credential found matching '{}'.", query);
    };

    Ok(())
}
