#![allow(unused)]
use argon2::Argon2;
use chacha20poly1305::{
    Key, XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use serde::{Deserialize, Serialize};

use errors::{DiskError, NetworkError, ParseError, WarpError};
use rand::{RngExt, distr::Alphanumeric};
use rand_core::{OsRng, RngCore};
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

// Common structs
pub mod crypto;
pub mod errors;
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

pub struct Files(PathBuf, FileData);

pub struct TransferContext {
    meta: FileData,
    file_handle: std::fs::File,
    bytes_tranferred: u64,
}

pub struct WarpApp<State> {
    pub role: State,
    pub config: Option<Config>,
}

pub struct HandshakeState {
    pub stream: TcpStream,
}

pub struct Config {
    worker_threads: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub name: String,
    pub file_size: u64,
    pub file_type: String,
}

pub fn parse_args(
    mut args_into_iter: impl IntoIterator<Item = String>,
) -> Result<CliArgs, ParseError> {
    let mut args_iter = args_into_iter.into_iter();

    let _program_name = args_iter.next();

    let command = args_iter
        .next()
        .ok_or_else(|| ParseError::Arg("No Command was provided".to_string()))?;

    match command.as_ref() {
        "send" => {
            let recv: SocketAddr = args_iter
                .next()
                .ok_or_else(|| ParseError::Arg("Target ip address was not provided".to_string()))?
                .parse()
                .map_err(|e| ParseError::InvalidIpAddress(e))?;

            let files: Vec<PathBuf> = args_iter.map(|f| PathBuf::from(f.trim())).collect();

            if files.is_empty() {
                return Err(ParseError::Arg(
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
                .ok_or_else(|| ParseError::Arg("No Port Number was provided".to_string()))?
                .parse()
                .map_err(|e| ParseError::PortParseError(e))?;

            Ok(CliArgs::Receive {
                listener_port: port,
            })
        }

        _ => Err(ParseError::Arg(
            "Please provide a valid command [send][receive]".to_string(),
        )),
    }
}

fn get_file_data(file_paths: Vec<PathBuf>) -> Result<Vec<Files>, DiskError> {
    let mut metadata: Vec<Files> = Vec::new();

    for f in file_paths {
        let path_str = f.to_string_lossy().to_string();

        let data = f.metadata().map_err(|io_err| DiskError::FileOpenError {
            filename: path_str.clone(),
            source: io_err,
        })?;

        let name = f
            .file_name()
            .ok_or(DiskError::InvalidFile(format!(
                "invalid file name {}",
                path_str.clone()
            )))?
            .to_string_lossy()
            .to_string();

        let file_type = f
            .extension()
            .ok_or(DiskError::InvalidFile(format!(
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

fn generate_hash() -> String {
    let hash: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    hash
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
