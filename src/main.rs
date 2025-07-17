mod models;
mod commands;
mod utils;
use models::credential::Credential;
use commands::add::add_credentail;


fn main() {
    println!("🔐 Welcome to your password manager CLI");
    let mut credentials: Vec<Credential> =Vec::new();
    let new_cred: Credential = add_credentail();
    println!("✅ Credential added:");
    println!("{:#?}", new_cred);
    credentials.push(new_cred);
}

