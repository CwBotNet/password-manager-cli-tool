use crate::commands::{get_master_password, get_storage};

pub fn run() -> anyhow::Result<()> {
    println!("🔐 Initializing new password vault...");

    let storage = get_storage()?;

    // Check if vault already exists
    if storage.vault_exists() {
        println!(
            "❌ Vault already exists at: {}",
            storage.get_vault_path().display()
        );
        println!("💡 Use other commands to manage your existing vault.");
        return Ok(());
    }

    // Get master Password with confirmation
    let master_password = get_master_password("Enter master password:")?;
    let confirm_password = get_master_password("Confirm master password:")?;

    if master_password != confirm_password{
        return Err(anyhow::anyhow!("❌ Password do not match"));
    }

    if master_password.len() < 8 {
        return Err(anyhow::anyhow!("‼️ Master password must be at least 8 characters"));
    }

    // Initialize vault 
    println!("🎉 Vault initialized successfully!");
    println!("📍 Location: {}", storage.get_vault_path().display());
    println!("\n💡 Next steps:");
    println!("  . Add your first credential: pwdmgr add \"Gmail Account\"");
    println!("  . List all credentials: pwdmgr list");
    println!("  . Get help: pwdmgr --help");

    Ok(())
}
