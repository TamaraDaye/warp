#![allow(unused)]

use spake2::{Ed25519Group, Identity, Password, Spake2};
use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*};
use std::io::BufReader;
use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};

fn main() -> std::io::Result<()> {
    let (tx, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut stream  = TcpStream::connect("127.0.0.1:8000")?;
    let mut write_stream = stream.try_clone()?;
    let mut read_stream = stream;
    let mut write_thread  = thread::spawn(move || {
        loop {
            let text = rx.recv().unwrap();
            write_stream.write_all(text.as_bytes()).unwrap();
        }
    });
    let read_thread = thread::spawn(move || {
        let mut buffer = BufReader::new(read_stream);
        let mut data = [0; 1024];
        let mut pointer : usize = 0;
        loop {
            let bytes_read = buffer.read(&mut data).unwrap();
            if bytes_read == 0{
                break
            }

            pointer += bytes_read;

            println!("read from tcp socket{:?}", String::from_utf8_lossy(&data[0..pointer]))
        }
    });
    let mut channel_thread = thread::spawn(move || {
        let stdin = std::io::stdin();
        loop {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            tx.send(input).unwrap();
        }
    });
    channel_thread.join().unwrap();
    write_thread.join().unwrap();
    read_thread.join().unwrap();
    Ok(())
}

