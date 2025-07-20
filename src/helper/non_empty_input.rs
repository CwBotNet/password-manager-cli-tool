use crate::utils::{Write, io};

pub fn get_non_empty_input(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdout().flush().expect("failed to flush the stdout");

        let _ = io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("‼️ Failed to read the input: {}", e));

        let trimmed = input.trim().to_string();

        if !trimmed.is_empty() {
            return trimmed;
        }

        println!("Input cannot be empty. plese try again.");
    }
}
