use crate::commands::{ensure_vault_exists, get_master_password, get_storage};
use anyhow::Result;
use console::{Style, Term};

pub fn run(show_password: bool) -> Result<()> {
    let storage = get_storage()?;
    ensure_vault_exists(&storage)?;

    // Get master password
    let master_password = get_master_password("🔐 Enter master password to unlock vault:")?;

    // Load Vault
    let vault = storage.load_vault(&master_password)?;

    if vault.credentials.is_empty() {
        println!("📭 No credentials found in vault.");
        println!("💡 Add your first credential with: pwdmgr add \"Service Name\"");
        return Ok(());
    }

    // Display credentials
    println!("📝 Credentials in vault ({}):\n", vault.credentials.len());

    let _term = Term::stdout();
    let header_style = Style::new().bold().cyan();
    let id_style = Style::new().dim();
    let _title_style = Style::new().bold();
    let warning_style = Style::new().red();

    for (index, credential) in vault.credentials.iter().enumerate() {
        println!(
            "{}",
            header_style.apply_to(format!("{}. {}", index + 1, credential.service))
        );
        println!(
            "  {}: {}",
            id_style.apply_to("ID"),
            id_style.apply_to(credential.id)
        );

        if let Some(username) = &credential.username {
            println!("  Username: {}", username);
        }

        if let Some(url) = &credential.url {
            println!("  URL: {}", url);
        }

        if show_password {
            println!(
                "  {}: {}",
                warning_style.apply_to("Password"),
                warning_style.apply_to(&credential.password)
            );
        } else {
            println!(
                "  Password: {} (use --show-passwords to reveal)",
                "........"
            );
        }

        if let Some(notes) = &credential.notes {
            println!("  Notes: {}", notes);
        }

        if !credential.tags.is_empty() {
            println!("  Tags: {}", credential.tags.join(",  "));
        }

        println!(
            "  Created: {}",
            credential.created_at.format("%Y-%m-%%d %H:%M UTC")
        );

        if credential.created_at != credential.updated_at {
            println!(
                "  Updated: {}",
                credential.updated_at.format("%Y-%m-%%d %H:%M UTC")
            );
        }

        println!();
    }

    if !show_password {
        println!("💡 Use --show-password flag to reveal passwords");
    }

    Ok(())
}
