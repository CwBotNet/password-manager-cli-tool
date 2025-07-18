use crate::models::credential::Credential;
use crate::utils::{Table,row};

pub fn list_credentials(credentials: &Vec<Credential>) {
    if credentials.is_empty() {
        println!("⚠️ NO SAVED CREDENTIALS FOUND.");
        return;
    }

    println!("\n 🔐 Saved credentials:");
    
    let mut table = Table::new();
    table.add_row(row!["ID", "Service", "Username", "Created At", "Notes"]);

    println!("{}", "-".repeat(75));

    for (i, cred) in credentials.iter().enumerate() {
        let notes = cred.notes.clone().unwrap_or("-".to_string());

        table.add_row(row![
            i + 1,
            cred.service.trim(),
            cred.username.trim(),
            cred.created_at.trim(),
            notes.trim()
            ]);
    }
    table.printstd();
    println!();  // for extra line
}
