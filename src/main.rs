mod commands;
mod models;
mod utils;
mod helper;


use commands::*;
use models::credential::Credential;
use utils::{io, process, Write};

fn main() {
    println!("🔐 Welcome to your password manager CLI");

    let mut credentials: Vec<Credential> = Vec::new();

    println!("for commands enter help / ?");
    loop {
        print!("> "); // Show prompt
        io::stdout().flush().unwrap();
        let mut user_command: String = String::new();
        io::stdin().read_line(&mut user_command).unwrap();

        match user_command.trim().to_lowercase().as_str() {
            "help" | "?" => {
                println!(
                    r#"
                        add : add new credential
                        list : show all credentials
                        search : search credential by tag or name
                        delete : delete credential
                        exit : exit from password manager cli"#
                );
            }
            "add" => {
                let new_cred: Credential = add::add_credentail();
                println!("✅ Credential added:");
                println!("{:#?}", new_cred);
                credentials.push(new_cred);
            }
            "list" => {
                list::list_credentials(&credentials);
            }
            "search" => {}
            "delete" => {}
            "exit" => process::exit(0),
            _ => {
                println!("‼️ invalid command");
            }
        }
    }
}
