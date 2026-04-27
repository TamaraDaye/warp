#![allow(unused)]
use rand::{distr::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt::write;
use std::io;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;

pub mod receiver;
pub mod sender;

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
pub enum AppRole {
    Client {
        listener: std::net::TcpListener,
        connections: Vec<Connection<ReceiverState>>,
    },
    Server {
        target_peer: std::net::SocketAddr,
        files: Vec<FileData>,
        connections: Vec<Connection<SenderState>>,
    },
}

//Represents possible states the Receiver of the files would be after connection established
pub enum ReceiverState {
    AwaitingHeader,
    AwaitingMetadata { metadata_size: u64 },
    RecevingFiles { transfer_metadata: TransferContext },
    Done,
    Error,
}

//Represents possible states the Sender would be after establishing the connection
pub enum SenderState {
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

pub struct WarpApp {
    app_role: AppRole,
    config: Config,
}

impl WarpApp {
    fn accept_connection(&self) -> Result<Connection<ReceiverState>, io::Error> {
        todo!()
    }

    fn start_connection(&self) -> Result<Connection<SenderState>, io::Error> {
        todo!()
    }
}

pub struct Config {
    worker_threads: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub name: String,
    pub file_size: u32,
    pub file_type: String,
}

pub struct Connection<T: ConnectionState> {
    peer_addr: SocketAddr,
    stream: TcpStream,
    state: T,
}

pub fn generate_hash() -> String {
    let hash: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    hash
}

#[derive(Debug)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32,
}

#[derive(Debug)]
pub enum ResultCode {
    Ok(u32),
    NotFound(u32),
    Teapot(u32),
    ErrorColor(Color),
}

fn parse_args(mut args_iter: impl Iterator<Item = String>) -> Result<CliArgs, Box<dyn Error>> {
    let command = args_iter.next();

    match command.as_deref() {
        Some("send") => {
            let mut target = args_iter.next();

            let recv_addr: SocketAddr;

            if let Some(addr) = target {
                recv_addr = addr.parse::<SocketAddr>()?;
            } else {
                return Err("Please specify a receiver address".into());
            }

            let files: Vec<PathBuf> = args_iter.map(|f| PathBuf::from(f)).collect();

            assert!(!files.is_empty());

            Ok(CliArgs::Send {
                target_peer: recv_addr,
                files,
            })
        }

        Some("get") => {
            let port: u16;
            if let Some(p) = args_iter.next() {
                port = p.parse()?;
            } else {
                return Err("Please specify a port to listen on and receive".into());
            };
            Ok(CliArgs::Receive {
                listener_port: port,
            })
        }

        _ => Err("Please provide a valid argument".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_send_command() {
        let mock_inputs = vec![
            "send".to_string(),
            "192.168.1.5:8080".to_string(),
            "data.txt".to_string(),
            "hello.py".to_string(),
            "welcome.py".to_string(),
        ];


        let result = parse_args(mock_inputs.into_iter());

        assert!(result.is_ok());

        if let Ok(CliArgs::Send { target_peer, files }) = result {
            assert_eq!(target_peer.ip().to_string(), "192.168.1.5");
            assert_eq!(files[0].to_str().unwrap(), "data.txt");
        } else {
            panic!("Expected CliArgs::Send variant!")
        }
    }

    #[test]
    fn test_valid_get_command() {
        let mock_inputs = vec!["get".to_string(), "4311".to_string()];

        let result = parse_args(mock_inputs.into_iter());

        assert!(result.is_ok());
    }

}

