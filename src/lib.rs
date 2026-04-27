#![allow(unused)]
use rand::{distr::Alphanumeric, Rng};
use std::fmt::write;
use std::io;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub enum ReceiverError {
    ConnectionDrop,
    CheckSum
}

#[derive(Debug)]
pub enum SenderError {
    ConnectionDrop,
    InvalidReceiver
}

impl std::fmt::Display for SenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SenderError::ConnectionDrop => { write!(f, "connection has been dropped with sender")}, 
            SenderError::InvalidReceiver => {write!(f, "Incorrect password provided by receiver")}
        }
    }
}

impl std::error::Error for SenderError{}

pub enum CliArgs {
    Send {
        target_pair: SocketAddr,
        files: Vec<PathBuf>
    },
    Receive {
        listener_port: u16
    }
}

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

pub enum ReceiverState {
    AwaitingHeader,
    AwaitingMetadata { metadata_size: u64 },
    RecevingFiles { transfer_metadata: TransferContext },
    Done,
    Error,
}

pub enum SenderState {
    SendingHeader,
    SendingMetadata { metadata_size: u64 },
    SendingFiles {},
    Done,
    Error,
}


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

pub struct Connection<T> {
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
    b: u32
}

#[derive(Debug)]
pub enum ResultCode {
    Ok(u32),
    NotFound(u32),
    Teapot(u32),
    ErrorColor(Color)
}
