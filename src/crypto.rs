use super::*;

pub fn encrypt_payload(
    encryption_key: &[u8; 32],
    nonce: &[u8; 24],
    plaintext: &[u8],
) -> Result<Vec<u8>, String> {

    let key = Key::from_slice(encryption_key);
    let formatted_nonce = XNonce::from_slice(nonce);

    let cipher = XChaCha20Poly1305::new(key);

    cipher.encrypt(formatted_nonce, plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))
}
pub fn decrypt_payload(
    encryption_key: &[u8; 32],
    nonce: &[u8; 24],
    ciphertext: &[u8],
) -> Result<Vec<u8>, String> {

    let key = Key::from_slice(encryption_key);
    let formatted_nonce = XNonce::from_slice(nonce);

    let cipher = XChaCha20Poly1305::new(key);

    cipher.decrypt(formatted_nonce, ciphertext)
        .map_err(|e| format!("Decryption failed or data corrupted: {}", e))
}

pub fn derive_encryption_key(password: String, salt: &[u8; 16]) -> [u8; 32] {
    let argon2 = Argon2::default();

    let mut key = [0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("Argon2 failed to allocate memory for hashing");

    key
}
