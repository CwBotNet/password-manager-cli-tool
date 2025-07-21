use crate::helper::non_empty_input::get_non_empty_input;
use crate::models::credential::Credential;
use crate::utils::crypto::{decrypt_password, encrypt_password};
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

    // TODO: Replace this with proper key derivation from master password
    let key: [u8; 32] = [42; 32];

    let (cypertext, nonce) = match encrypt_password(password.as_bytes(), &key, service.as_bytes()) {
        Ok((ct, n)) => (ct, n),
        Err(e) => {
            eprint!("Encryption failed: {:?}", e);
            panic!("Failed to encrypt password");
        }
    };

    let stored_ciphertext = cypertext;
    let stored_nonce = nonce;

    match decrypt_password(&stored_ciphertext, &key, &stored_nonce, service.as_bytes()) {
        Ok(decrypt_password) => {
            println!("Decryption successful!");
            println!(
                "Recovered Password: {}",
                String::from_utf8_lossy(&decrypt_password)
            );
            assert_eq!(decrypt_password, password.as_bytes());
        }
        Err(_) => {
            println!(
                "Decryption failed! The data may have been tampered with or the key is wrong."
            );
        }
    }

    let created_at = Utc::now();

    Credential {
        service: service,
        username: username.trim().to_string(),
        password: password.trim().to_string(),
        created_at,
        notes,
    }
}
