#![allow(unused)]

use spake2::{Ed25519Group, Identity, Password, Spake2};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut stream  = TcpStream::connect("127.0.0.1:8000")?;
    let mut buffer = BufReader::new(stream);
    let mut message_size = [0; 1];
    let mut file_name: Vec<u8> = vec![];
    let mut file_type: Vec<u8> = vec![];
    buffer.read_exact(&mut message_size)?;
    let bytes_read: u8 = 0;
    while bytes_read < message_size[0]{
        buffer.read_exact
    }
    // let mut byte_count: usize = 0;
    // let mut buf  = [0; 1024];
    // while byte_count < 11 {
    //     let bytes_read = stream.read(&mut buf[byte_count..])?;
    //     if bytes_read == 0 {
    //         break
    //     };
    //     byte_count += bytes_read;
    // }
    // println!("{byte_count}: read from the socket");
    // println!("{:?}", &buf[0..12]);
    let (s1, outbound_msg) = Spake2::<Ed25519Group>::start_b(
       &Password::new(b"password"),
       &Identity::new(b"client id string"),
       &Identity::new(b"server id string"));
    Ok(())
}
