use std::io::Read;
use std::net::TcpStream;

use rand_core::{OsRng, RngCore};

use crate::crypto::{derive_encryption_key, encrypt_payload};
use crate::{errors::NetworkError, generate_hash, Files};

use super::{FileData, WarpApp};

pub struct SenderInitial {
    pub target_peer: std::net::SocketAddr,
    pub files: Vec<Files>,
}

pub struct SenderHandshaking {
    pub stream: TcpStream,
    pub files: Vec<Files>,
    pub salt: [u8; 16],
}

pub struct SenderStreaming {
    pub stream: TcpStream,
    // pub cipher: XChaCha20poly1305,
    pub nonce: [u8; 24],
    pub file_queue: Vec<Files>,
    key: [u8; 32],
}

impl WarpApp<SenderInitial> {
    pub fn connect(self) -> Result<WarpApp<SenderHandshaking>, NetworkError> {
        let stream =
            TcpStream::connect(self.role.target_peer).map_err(NetworkError::ReceiverError)?;

        Ok(WarpApp {
            role: SenderHandshaking {
                stream,
                files: self.role.files,
                salt: [0u8; 16], // To be filled...
            },
            config: self.config,
        })
    }
}
impl WarpApp<SenderHandshaking> {
    fn handshake(mut self) -> Result<WarpApp<SenderStreaming>, NetworkError> {
        let password = generate_hash();
        self.role
            .stream
            .read_exact(&mut self.role.salt)
            .map_err(NetworkError::ReceiverError)?;
        let encryption_key = derive_encryption_key(password, &self.role.salt);
        let mut nonce = [0u8; 24];

        OsRng.fill_bytes(&mut nonce);
        Ok(WarpApp {
            role: SenderStreaming {
                stream: self.role.stream,
                nonce,
                file_queue: self.role.files,
                key: encryption_key,
            },
            config: self.config,
        })
    }
}

impl WarpApp<SenderStreaming> {
    fn send_header(&mut self) -> Result<(), NetworkError> {
        let size_of_metadata =
            std::mem::size_of_val(&self.role.file_queue[self.role.file_queue.len() - 1].1);

        Ok(())
    }
}
