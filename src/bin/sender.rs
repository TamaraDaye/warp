#![allow(unused)]
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::thread;
use spake2::{Ed25519Group, Identity, Password, Spake2};
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, Sender};

fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    match listener.accept() {
        Ok((mut stream, addr)) => {
            let (tx, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();
            //clone the tcp stream and create two threads to utilize two way of the stream
            let mut write_stream = stream.try_clone()?;
            let read_thread = thread::spawn(move || {
                let mut buffer = BufReader::new(stream);
                let mut pointer : usize = 0;
                let mut data = [0; 1024];

                loop {
                    let bytes_read = buffer.read(&mut data).unwrap();
                    if bytes_read == 0 {
                        break
                    }
                    pointer += bytes_read;
                    println!("Read data from {addr} {:?}", String::from_utf8_lossy(&data[0..pointer]))
                }
            });

            let write_thread = thread::spawn(move || {
                loop {
                    let text = rx.recv().unwrap();
                    write_stream.write_all(text.as_bytes());
                }
            });

            let channel_thread = thread::spawn(move || {
                let stdin = std::io::stdin();
                loop {
                    let mut input= String::new();
                    stdin.read_line(&mut input).unwrap();
                    tx.send(input).unwrap();
                }
            });

            channel_thread.join().unwrap();
            read_thread.join().unwrap();
            write_thread.join().unwrap();
        },
        Err(e) => println!("couldn't get a client : {e:?}")
    };


    Ok(())
}

 
