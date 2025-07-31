use crate::{
    commands::{ensure_vault_exists, get_master_password, get_storage},
    models::credential,
};
use anyhow::Result;

pub fn run(term: String) -> anyhow::Result<()> {
    let storage = get_storage()?;
    ensure_vault_exists(&storage)?;

    //Prompt user for master password
    let master_password = get_master_password("🔐 Enter master password to unlock vault:")?;

    // Load vault
    let vault = storage.load_vault(&master_password)?;

    // Search credentials using matches_search method
    let results: Vec<_> = vault.search_credential(&term);

    if results.is_empty() {
        println!("🔍 No credentials found matching '{}'.", term);
    } else {
        println!("🔍 Found {} matching credential(s):", results.len());
        for credential in results {
            println!("{}", credential.display_safe());
            println!("---");
        }
    }
    Ok(())
}
