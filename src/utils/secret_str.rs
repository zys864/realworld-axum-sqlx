use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(s: impl std::fmt::Display) -> Self {
        Self(s.to_string())
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("***")
    }
}
impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("***")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_trait() {
        let s = SecretString::new("password");
        println!("{:?}", s);
        println!("{}", s);
    }
    #[test]
    fn test_serde() {
        let s = SecretString::new("password");
        let out = serde_json::to_string(&s).unwrap();
        // assert_eq!("password",out);
        println!("{}", out);
        let des = serde_json::from_str::<SecretString>(r#""password""#).unwrap();
        println!("{}", des.0)
    }
}
