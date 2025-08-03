use dialoguer::{Confirm, Input};

use crate::commands::{ensure_vault_exists, get_master_password, get_storage};
use crate::models::credential::Credential;
use crate::utils::generator::genrate_password;
// use crate::utils::crypto::{decrypt_password, encrypt_password};

pub fn run(
    title: String,
    username: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    generate: bool,
) -> anyhow::Result<()> {
    println!("➕ Adding new credential: {}", title);

    let storage = get_storage()?;
    ensure_vault_exists(&storage)?;

    // Get master password
    let master_password = get_master_password("🔐 Enter master password to unlock vault")?;

    // Load existing vault
    let mut vault = storage.load_vault(&master_password)?;

    // Collect credentails information
    let _username: Option<String> = match username {
        Some(u) => Some(u),
        None => {
            let input: String = Input::new()
                .with_prompt("Username (optional, press Enter to skip)")
                .allow_empty(true)
                .interact_text()?;
            if input.is_empty() { None } else { Some(input) }
        }
    };

    let url = match url {
        Some(u) => Some(u),
        None => {
            let input: String = Input::new()
                .with_prompt("URL (optional, press Enter to skip)")
                .allow_empty(true)
                .interact_text()?;
            if input.is_empty() { None } else { Some(input) }
        }
    };

    let notes = match notes {
        Some(n) => Some(n),
        None => {
            let input: String = Input::new()
                .with_prompt("Notes (optional, press Enter to skip)")
                .allow_empty(true)
                .interact_text()?;
            if input.is_empty() { None } else { Some(input) }
        }
    };

    // Handle password
    let password = if generate {
        let length: usize = Input::new()
            .with_prompt("password length")
            .default(16)
            .interact_text()?;

        let include_symbols = Confirm::new()
            .with_prompt("Include symbols?")
            .default(true)
            .interact()?;

        let generated = genrate_password(length, include_symbols)?;
        println!(" Genrated password: {}", generated);
        generated
    } else {
        get_master_password("Enter password for the credential:")?
    };

    // Create credental
    let mut credential = Credential::new(title, password);

    if let Some(u) = url {
        credential = credential.with_url(u);
    }

    if let Some(n) = notes {
        credential = credential.with_notes(n)
    }

    // Add to vault and save
    vault.add_credentail(credential);
    storage.save_vault(&vault, &master_password)?;

    println!("✅ Credential added successfully!");
    println!("📊 Total credentials in vault: {}", vault.credentials.len());

    Ok(())
}
