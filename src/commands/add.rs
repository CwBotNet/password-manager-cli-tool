use crate::models::credential::Credential;
use crate::utils::{Local,io};

pub fn add_credentail()-> Credential{
    println!("Enter service provide: ");
    let mut service = String::new();
    io::stdin().read_line(&mut service).unwrap();

    println!("Enter username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    println!("Enter your password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    println!("Any addintional notes? (press enter to skip): ");
    let mut notes_input = String::new();
    io::stdin().read_line(&mut notes_input).unwrap();

    let notes = if notes_input.trim().is_empty(){
        None
    }else{
        Some(notes_input.trim().to_string())
    };

    let created_at = Local::now().to_string();

    Credential{
        service: service.trim().to_string(),
        username: username.trim().to_string(),
        password: password.trim().to_string(),
        created_at,
        notes,
    }

}