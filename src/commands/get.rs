use crate::{
    commands::{ensure_vault_exists, get_master_password, get_storage},
    models::credential,
    storage,
};
use anyhow::Result;
use console::Style;
use uuid::Uuid;

pub fn run(query: String, copy: bool) -> anyhow::Result<()> {
    let storage = get_storage()?;
    ensure_vault_exists(&storage)?;

    // Get master password
    let master_password = get_master_password("🔐 Enter master password to unlock vault:")?;

    // Load vault
    let vault = storage.load_vault(&master_password)?;

    // Find credential
    let credential = if let Ok(uuid) = Uuid::parse_str(&query) {
        // Search by UUID
        vault.find_credential(&uuid)
    } else {
        // Search by title, username, or URL
        vault.credentials.iter().find(|c| {
            c.service.to_lowercase().contains(&query.to_lowercase())
                || c.username.as_ref().map_or(false, |u| {
                    u.to_lowercase().contains(&query.to_ascii_lowercase())
                })
                || c.url
                    .as_ref()
                    .map_or(false, |u| u.to_lowercase().contains(&query.to_lowercase()))
        })
    };

    match credential {
        Some(cred) => {
            let title_style = Style::new().bold().green();
            let lable_style = Style::new().bold();
            let password_style = Style::new().red();

            println!(
                "{}",
                title_style.apply_to(format!("🔍 Found: {}", cred.service))
            );
            println!();

            println!("{}: {}", lable_style.apply_to("ID"), cred.id);

            if let Some(username) = &cred.username {
                println!("{}: {}", lable_style.apply_to("Username"), username);
            }

            println!(
                "{}: {}",
                lable_style.apply_to("Password"),
                password_style.apply_to(&cred.password)
            );

            if let Some(url) = &cred.url {
                println!("{}: {}", lable_style.apply_to("URL"), url);
            }

            if let Some(notes) = &cred.notes {
                println!("{}: {}", lable_style.apply_to("Notes"), notes);
            }

            if !cred.tags.is_empty() {
                println!("{}: {}", lable_style.apply_to("Tags"), cred.tags.join(", "));
            }

            println!(
                "{}: {}",
                lable_style.apply_to("Created"),
                cred.created_at.format("%Y-%m-%d %H:%M UTC")
            );

            if cred.created_at != cred.updated_at {
                println!(
                    "{}: {}",
                    lable_style.apply_to("Updated"),
                    cred.updated_at.format("%Y-%m-%d %H:%M UTC")
                );
            }

            // Copy to clipboard if requested
            if copy {

                #[cfg(feature = "clipboard")]
                {
                    use clipboard::{ClipboardContext, ClipboardProvider};
                    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
                    ctx.set_contents(cred.password.clone())?;
                    println!("\n📝 Password copied to clipboard!");
                }

                #[cfg(not(feature = "clipboard"))]
                {
                    println!("\n❌ Clipboard feature not enabled. Password shown above.");
                }
            }
        },
        None => {
            println!("❌ No credentails found matching: '{}'", query);
            println!("💡 Use 'pwdmgr list' to see all credentails");
            println!("💡 Use 'pwdmgr search <term>' to search more broadly");
        }
    }

    Ok(())
}
