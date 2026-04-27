#![allow(unused)]

use std::error::Error;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::{env, io};

use cli_tool::*;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}


fn start(conifg: Config) -> Result<WarpApp, io::Error> {
    todo!()
}

fn get_files(files: Vec<String>) -> Result<Vec<FileData>, io::Error> {
    todo!()
}
