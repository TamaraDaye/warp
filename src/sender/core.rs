use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
use chacha20poly1305::aead::rand_core::RngCore;

fn accept() {}
