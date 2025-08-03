use anyhow::Result;
use rand::{RngCore,rng};
/// Genrate a strong. random Password.
///
/// # Arguments
/// - `length`: Desired length of password
/// - `include_symbols`: if true, includes [!@#$%^&*_+-=] in pool
pub fn genrate_password(length: usize, include_symbols: bool) -> Result<String> {
    if length < 8 {
        return Err(anyhow::anyhow!(
            "‼️ password too short (must be at least 8 characters)"
        ));
    }

    let mut charset =
        String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let symbols = "!@#$%^&*()_-+=[]{}|;:,.<>?/";

    if include_symbols {
        charset.push_str(symbols);
    }

    let mut rng = rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.next_u32() as usize % charset.len();
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() {
        let pass = genrate_password(20, true).unwrap();
        assert_eq!(pass.len(), 20);
    }

    #[test]
    fn test_generate_password_no_symbols() {
        let pass = genrate_password(32, false).unwrap();
        assert!(pass.chars().all(|c| c.is_alphanumeric()))
    }
}
