use crate::helper::non_empty_input::get_non_empty_input;
use crate::models::credential::Credential;
use crate::utils::{Utc, io};

pub fn add_credentail() -> Credential {
    let service = get_non_empty_input("Enter service provider: ");
    let username = get_non_empty_input("Enter username: ");
    let password = get_non_empty_input("Enter password: ");

    println!("Any addintional notes? (press enter to skip): ");
    let mut notes_input = String::new();
    io::stdin().read_line(&mut notes_input).unwrap();

    let notes = if notes_input.trim().is_empty() {
        None
    } else {
        Some(notes_input.trim().to_string())
    };

    let created_at = Utc::now();

    Credential {
        service: service,
        username: username.trim().to_string(),
        password: password.trim().to_string(),
        created_at,
        notes,
    }
}
