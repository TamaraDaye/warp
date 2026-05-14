use rand_core::{OsRng, RngCore};

use crate::{crypto::derive_encryption_key, errors::NetworkError};

use super::WarpApp;
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

pub struct ReceiverInitial {
    pub target_peer: std::net::SocketAddr,
    pub password: String
}

pub struct ReceiverHandshaking {
    pub stream: TcpStream,
    pub target_peer: std::net::SocketAddr,
    pub salt: [u8; 16],
    pub password: String
}

pub struct ReceiverStreaming {
    pub stream: TcpStream, 
    pub target_peer: std::net::SocketAddr,
    key: [u8; 32],
    nonce: [u8; 24]
}


impl WarpApp<ReceiverInitial> {
    fn connect(self) -> Result<WarpApp<ReceiverHandshaking>, NetworkError> {
        let stream =
            TcpListener::bind(self.role.target_peer).map_err(NetworkError::ReceiverError)?;

        let (socket, addr) = stream.accept().map_err(NetworkError::ReceiverError)?;

        let mut salt = [0u8; 16];

        OsRng.fill_bytes(&mut salt);

        Ok(WarpApp {
            role: ReceiverHandshaking {
                stream: socket,
                target_peer: addr,
                password: self.role.password,
                salt,
            },
            config: self.config,
        })
    }
}

impl WarpApp<ReceiverHandshaking> {
    fn handshake(mut self) -> Result<WarpApp<ReceiverStreaming>, NetworkError> {
        self.role.stream.write_all(&self.role.salt).map_err(NetworkError::ReceiverError)?;
        let key = derive_encryption_key(self.role.password, &self.role.salt);

        Ok(WarpApp {
            role: ReceiverStreaming {
                key, 
                stream: self.role.stream, 
                nonce: [0u8; 24], 
                target_peer: self.role.target_peer, 
            }, 
            config: self.config
        })
    }
}

impl WarpApp<ReceiverStreaming> {
    fn receiver_header(&mut self) -> Result<u64, NetworkError> {
        let mut file_metadata_size: [u8; 64]  = [0; 64];

        self.role.stream.read_exact(&mut file_metadata_size)?;

        file_metadata_size.into()
        
    }
}
