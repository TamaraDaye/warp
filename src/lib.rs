#![allow(unused)]
use rand::{Rng, distr::Alphanumeric};
use std::error::Error;
use std::io::prelude::*;

pub enum Command {
    Send,
    Receive,
}

pub struct Config {
    command: Command,
    files: Option<Vec<String>>,
}

pub struct Filedata{
    pub name: String,
    pub file_size: u32, 
    pub file_type: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please specify a command");
        }

        match args[1].as_str() {
            "send" => {
                if args.len() < 3 {
                    Err("Invalid arguments: select files for transfer")
                } else {
                    Ok(Config {
                        command: Command::Send,
                        files: Some(args[1..].to_vec()),
                    })
                }
            }

            "receive" => Ok(Config {
                command: Command::Receive,
                files: None,
            }),

            _ => Err("Invalid arguments provided"),
        }
    }
}

pub fn generate_hash() -> String {
    let hash: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    hash
}
