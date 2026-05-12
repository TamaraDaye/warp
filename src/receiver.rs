use crate::errors::NetworkError;

use super::WarpApp;
use std::net::{TcpListener, TcpStream};

pub struct ReceiverInitial {
    pub target_peer: std::net::SocketAddr,
}

pub struct ReceiverHandshaking {
    pub stream: TcpStream,
    pub target_peer: std::net::SocketAddr,
    pub salt: [u8; 16],
}

impl WarpApp<ReceiverInitial> {
    fn connect(self) -> Result<WarpApp<ReceiverHandshaking>, NetworkError> {
        let stream =
            TcpListener::bind(self.role.target_peer).map_err(NetworkError::ReceiverError)?;

        let (socket, addr) = stream.accept().map_err(NetworkError::ReceiverError)?;

        Ok(WarpApp {
            role: ReceiverHandshaking {
                stream: socket,
                target_peer: addr,
                salt: [0u8; 16],
            },
            config: self.config,
        })
    }
}
