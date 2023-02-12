use argon2::{self, Config, ThreadMode, Variant, Version};
use once_cell::sync::Lazy;
// encrypt

static ARGON2_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    variant: Variant::Argon2i,
    version: Version::Version13,
    mem_cost: 65536,
    time_cost: 10,
    lanes: 4,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 32,
});
static SALT: Lazy<String> =
    Lazy::new(|| std::env::var("SALT").expect("salt env not be seted"));
pub fn hash(password: impl AsRef<str>) -> argon2::Result<String> {
    argon2::hash_encoded(
        password.as_ref().as_bytes(),
        SALT.as_bytes(),
        &ARGON2_CONFIG,
    )
}
pub fn verify_hash<T>(password: T, hash: T) -> argon2::Result<bool>
where
    T: AsRef<str>,
{
    argon2::verify_encoded(hash.as_ref(), password.as_ref().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        std::env::set_var("SALT", "dsdsdweasff");
        let password = "password".to_string();
        let hashed_password = hash(password.clone()).unwrap();
        let match_result = verify_hash(password, hashed_password).unwrap();
        assert!(match_result)
    }
}
