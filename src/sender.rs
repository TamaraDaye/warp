use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;

use rand_core::{OsRng, RngCore};

use crate::TransferContext;
use crate::crypto::{derive_encryption_key, encrypt_payload};
use crate::{errors::NetworkError, generate_hash, File};

use super::{FileData, WarpApp};

pub struct SenderInitial {
    pub target_peer: std::net::SocketAddr,
    pub files: VecDeque<File>,
}

pub struct SenderHandshaking {
    pub stream: TcpStream,
    pub files: VecDeque<File>,
    pub salt: [u8; 16],
}

pub struct SenderStreaming {
    pub stream: TcpStream,
    // pub cipher: XChaCha20poly1305,
    pub nonce: [u8; 24],
    pub file_queue: VecDeque<File>,
    key: [u8; 32],
    pub active_file: Option<TransferContext>
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
                active_file: None
            },
            config: self.config,
        })
    }
}

impl WarpApp<SenderStreaming> {
    fn start_transfer(&self){}


    fn send_header(&mut self) -> Result<(), NetworkError> {
        let file = self.role.file_queue.pop_front();

        if let Some(File(path, filedata)) = file {
            let f = std::fs::File::open(path).map_err(NetworkError::ReceiverError)?;

            if self.role.active_file.is_none() {
                self.role.active_file = Some(TransferContext {
                    file_handle: f, 
                    bytes_tranferred: 0
                });
            }

            let metadata = postcard::to_vec(&filedata)?;
        }
    }

pub fn increment_nonce(&mut self) {
    for byte in self.role.nonce.iter_mut() {
        let (new_value, did_overflow) = byte.overflowing_add(1);
        
        *byte = new_value;
        if !did_overflow {
            break;
        }
    }
}
}
