use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub fn hash_password(raw: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(raw.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(raw: &str, encoded: &str) -> bool {
    let parsed = match PasswordHash::new(encoded) {
        Ok(value) => value,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(raw.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_password_rejects_wrong_secret() {
        let encoded = hash_password("super-secret-password").unwrap();
        assert!(!verify_password("wrong-password", &encoded));
    }

    #[test]
    fn verify_password_accepts_correct_secret() {
        let encoded = hash_password("super-secret-password").unwrap();
        assert!(verify_password("super-secret-password", &encoded));
    }
}
