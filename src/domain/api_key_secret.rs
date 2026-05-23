use sha2::{Digest, Sha256};
use uuid::Uuid;

pub const API_KEY_HASH_HEX_LENGTH: usize = 64;

pub fn generate_api_key_value() -> String {
    format!("rk_{}", Uuid::new_v4().simple())
}

pub fn hash_api_key_value(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

pub fn is_hashed_api_key_value(value: &str) -> bool {
    value.len() == API_KEY_HASH_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::{API_KEY_HASH_HEX_LENGTH, hash_api_key_value, is_hashed_api_key_value};

    #[test]
    fn hashes_plaintext_to_hex_digest() {
        let hash = hash_api_key_value("rk_example_secret");

        assert_eq!(hash.len(), API_KEY_HASH_HEX_LENGTH);
        assert_ne!(hash, "rk_example_secret");
        assert!(is_hashed_api_key_value(&hash));
    }

    #[test]
    fn rejects_legacy_uuid_plaintext() {
        assert!(!is_hashed_api_key_value(
            "550e8400-e29b-41d4-a716-446655440000"
        ));
    }
}
