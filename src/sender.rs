use std::net::TcpStream;

use crate::{Files, errors::NetworkError};

use super::{WarpApp, FileData};

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
}

impl WarpApp<SenderInitial> {
    pub fn connect(self) -> Result<WarpApp<SenderHandshaking>, NetworkError> {
        let stream = TcpStream::connect(self.role.target_peer).map_err(NetworkError::ReceiverError)?;

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
    fn handshake(self) -> Result<WarpApp<SenderStreaming>, NetworkError> {
        todo!()
    }
}

impl WarpApp<SenderStreaming> {}
