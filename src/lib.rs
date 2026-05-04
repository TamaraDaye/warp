#![allow(unused)]
use argon2::Argon2;
use argon2::password_hash::rand_core::RngCore;
use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{Key, KeyInit, XChaCha20Poly1305, XNonce};
use rand::RngCore;
use rand::{distr::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{format, write};
use std::io;
use std::io::prelude::*;
use std::net::{AddrParseError, SocketAddr, TcpListener, TcpStream};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::{env, thread};
use thiserror::Error;

pub mod receiver;
pub mod sender;

#[derive(Debug)]
pub enum CliArgs {
    Send {
        target_peer: SocketAddr,
        files: Vec<PathBuf>,
    },
    Receive {
        listener_port: u16,
    },
}

//Enum for specifying which type of operation the bootstrapped app will be performing

pub struct Files(pub PathBuf, pub FileData);

pub struct FileSender {
    target_peer: std::net::SocketAddr,
    files: Vec<Files>,
    connections: Vec<Connection<SenderState>>,
    state: SenderState,
}

pub struct FileReceiver {
    listener: std::net::TcpListener,
    connections: Vec<Connection<ReceiverState>>,
    state: ReceiverState,
}

//Represents possible states the Receiver of the files would be after connection established
pub enum ReceiverState {
    Initial,
    AwaitingHeader,
    AwaitingMetadata { metadata_size: u64 },
    RecevingFiles { transfer_metadata: TransferContext },
    Done,
    Error,
}

//Represents possible states the Sender would be after establishing the connection
pub enum SenderState {
    Initial,
    SendingHeader,
    SendingMetadata { metadata_size: u64 },
    SendingFiles {},
    Done,
    Error,
}

pub trait ConnectionState {}

impl ConnectionState for ReceiverState {}
impl ConnectionState for SenderState {}

pub struct TransferContext {
    meta: FileData,
    file_handle: std::fs::File,
    bytes_tranferred: u64,
}

#[derive(Debug, Error)]
pub enum WarpError {
    #[error("could not open file '{filename}'")]
    FileOpenError {
        filename: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Could not read file '{filename}'")]
    FileUnreadableError {
        filename: String,
        #[source]
        source: std::io::Error,
    },

    #[error("could not parse port number")]
    PortParseError(#[source] ParseIntError),

    #[error("Invalid address could not parse")]
    ReceiverError(#[source] std::io::Error),

    #[error("Not a valid file")]
    InvalidFile(String),

    #[error("")]
    ParseError(String),
}

pub enum NetworkError {}

pub enum ParseError {}

pub enum DiskError {}

pub struct Config {
    worker_threads: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub name: String,
    pub file_size: u64,
    pub file_type: String,
}

pub struct Connection<T: ConnectionState> {
    peer_addr: SocketAddr,
    stream: TcpStream,
    state: T,
}
pub struct WarpApp<Role> {
    pub role: Role,
    pub config: Option<Config>,
}

impl WarpApp<FileSender> {
    fn new(
        target_peer: std::net::SocketAddr,
        files: Vec<std::path::PathBuf>,
    ) -> Result<Self, WarpError> {
        let metadata = get_file_data(files)?;

        let app = WarpApp {
            role: FileSender {
                target_peer,
                files: metadata,
                connections: vec![],
                state: SenderState::Initial,
            },
            config: None,
        };

        Ok(app)
    }
}

impl WarpApp<FileSender> {
    fn send(&self, password: String) -> Result<(), io::Error> {
        let mut stream = std::net::TcpStream::connect(self.role.target_peer)?;

        let mut salt = [0u8; 16];

        stream.read_exact(&mut salt)?;

        let sender_key = derive_encryption_key(&password, &salt);

        let mut write_sream = stream.try_clone();

        Ok(())
    }
}

impl WarpApp<FileReceiver> {
    fn new(listener_port: u16) -> Result<Self, WarpError> {
        let addr = std::net::TcpListener::bind(format!("127.0.0.1:{}", listener_port))
            .map_err(WarpError::ReceiverError)?;
        let app = WarpApp {
            role: FileReceiver {
                listener: addr,
                connections: vec![],
                state: ReceiverState::Initial,
            },
            config: None,
        };

        Ok(app)
    }

    fn receive(&mut self) -> Result<(), WarpError> {
        let password = generate_hash();

        let mut salt = [0u8; 16];

        rand::rng().fill_bytes(&mut salt);

        let receiver_key = derive_encryption_key(&password, &salt);

        let (mut stream, addr) = self
            .role
            .listener
            .accept()
            .map_err(WarpError::ReceiverError)?;

        stream.write_all(&salt).unwrap();

        Ok(())
    }
}

pub fn parse_args(
    mut args_into_iter: impl IntoIterator<Item = String>,
) -> Result<CliArgs, WarpError> {
    let mut args_iter = args_into_iter.into_iter();

    let _program_name = args_iter.next();

    let command = args_iter
        .next()
        .ok_or_else(|| WarpError::ParseError("No Command was provided".to_string()))?;

    match command.as_ref() {
        "send" => {
            let recv: SocketAddr = args_iter
                .next()
                .ok_or_else(|| {
                    WarpError::ParseError("Target ip address was not provided".to_string())
                })?
                .parse()
                .map_err(|e| WarpError::ParseError(format!("Target ip unavailable: {}", e)))?;

            let files: Vec<PathBuf> = args_iter.map(|f| PathBuf::from(f.trim())).collect();

            if files.is_empty() {
                return Err(WarpError::ParseError(
                    "Please provide a file(s) to transfer".to_string(),
                ));
            }

            Ok(CliArgs::Send {
                target_peer: recv,
                files,
            })
        }

        "receive" => {
            let port: u16 = args_iter
                .next()
                .ok_or_else(|| WarpError::ParseError("No Port Number was provided".to_string()))?
                .parse()
                .map_err(|e| {
                    WarpError::ParseError(format!("Invalid port number provided: {}", e))
                })?;

            Ok(CliArgs::Receive {
                listener_port: port,
            })
        }

        _ => Err(WarpError::ParseError(
            "Please provide a valid command [send][receive]".to_string(),
        )),
    }
}

fn get_file_data(file_paths: Vec<PathBuf>) -> Result<Vec<Files>, WarpError> {
    let mut metadata: Vec<Files> = Vec::new();

    for f in file_paths {
        let path_str = f.to_string_lossy().to_string();

        let data = f.metadata().map_err(|io_err| WarpError::FileOpenError {
            filename: path_str.clone(),
            source: io_err,
        })?;

        let name = f
            .file_name()
            .ok_or(WarpError::InvalidFile(format!(
                "File doesn't exist {}",
                path_str
            )))?
            .to_string_lossy()
            .to_string();

        let file_type = f
            .extension()
            .ok_or(WarpError::InvalidFile(format!(
                "File type cannot be deduced {}",
                path_str
            )))?
            .to_string_lossy()
            .to_string();

        metadata.push(Files(
            f.to_owned(),
            FileData {
                name,
                file_type,
                file_size: data.len(),
            },
        ));
    }

    Ok(metadata)
}

fn derive_encryption_key(password: &String, salt: &[u8; 16]) -> [u8; 32] {
    let argon2 = Argon2::default();

    let mut key = [0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("Argon2 failed to allocate memory for hashing");

    key
}

fn generate_hash() -> String {
    let hash: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    hash
}

fn encrypt_payload(password_hash: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, [u8; 24]) {
    let key = Key::from_slice(password_hash);

    let cipher = XChaCha20Poly1305::new(key);

    let mut raw_nonce = [0u8; 24];

    OsRng.fill_bytes(&mut raw_nonce);

    let nonce = XNonce::from_slice(&raw_nonce);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failed");

    (ciphertext, raw_nonce)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    #[test]
    fn test_valid_send_command() -> Result<()> {
        let mock_inputs = vec![
            "send".to_string(),
            "4311".to_string(),
            "data.txt".to_string(),
            "hello.py".to_string(),
            "welcome.py".to_string(),
        ];

        let result = parse_args(mock_inputs);

        assert!(result.is_ok());

        if let Ok(CliArgs::Send {
            ref target_peer,
            ref files,
        }) = result
        {
            assert_eq!(target_peer.ip().to_string(), "192.168.1.5");
            assert_eq!(files[0].to_str().unwrap(), "data.txt");
            println!("{:?}", result)
        }
        Ok(())
    }

    #[test]
    fn test_valid_get_command() -> Result<()> {
        let mock_inputs = vec!["receive".to_string(), "4311".to_string()];

        let result = parse_args(mock_inputs)?;

        Ok(())
    }

    #[test]
    fn parse_files() -> Result<()> {
        Ok(())
    }
}
