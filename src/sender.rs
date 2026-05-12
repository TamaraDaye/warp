use std::net::TcpStream;

use crate::WarpError;

use crate::Files;

use super::WarpApp;

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
    pub fn connect(self) -> Result<WarpApp<SenderHandshaking>, WarpError> {
        let stream = TcpStream::connect(self.role.target_peer)?;

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
impl WarpApp<SenderHandshaking> {}

impl WarpApp<SenderStreaming> {}
