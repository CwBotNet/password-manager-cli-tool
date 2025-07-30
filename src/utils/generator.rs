use anyhow::Result;
use rand::{distr::{Alphanumeric, SampleString},thread_rng,RngCore};
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
    let sumbols = "!@#$%^&*()_-+=[]{}|;:,.<>?/";

    let mut rng =thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.next_u32() as usize % charset.len();
            charset.chars().nth(idx).unwrap()
        })
        .collect();

        Ok(password)
}
