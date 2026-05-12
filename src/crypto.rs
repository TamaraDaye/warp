use super::*;

fn encrypt_payload(
    password_hash: &[u8; 32],
    plaintext: &[u8],
    nonce: [u8; 24],
) -> (Vec<u8>, [u8; 24]) {
    let key = Key::from_slice(password_hash);

    let cipher = XChaCha20Poly1305::new(key);

    let mut raw_nonce = [0u8; 24];

    OsRng.fill_bytes(&mut raw_nonce);

    let nonce = XNonce::from_slice(&raw_nonce);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failed");

    (ciphertext, raw_nonce)
}

fn derive_encryption_key(password: &String, salt: &[u8; 16]) -> [u8; 32] {
    let argon2 = Argon2::default();

    let mut key = [0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("Argon2 failed to allocate memory for hashing");

    key
}
