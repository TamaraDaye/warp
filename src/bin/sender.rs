#![allow(unused)]
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::thread;
use spake2::{Ed25519Group, Identity, Password, Spake2};
use cli_tool::{generate_hash, Filedata};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let password: String = generate_hash();
    println!("{password}");
    let (s1, outbound_msg) = Spake2::<Ed25519Group>::start_a(
        &Password::new(b"password"),
        &Identity::new(b"client id string"),
        &Identity::new(b"server id string"),
    );

    let mut file = Filedata{
        name: String::from("Hello.txt"),
        file_type: String::from(".text"),
        file_size: 78
    };

    let mut data: Vec<u8> = vec![18];
    data.push(9);
    data.extend_from_slice(file.name.as_bytes());
    data.push(5);
    data.extend_from_slice(file.file_type.as_bytes());

     loop {
        match listener.accept() {
            Ok((mut stream, addr)) => {
                println!("new client: {addr:?}");
                stream.write_all(password.as_bytes());
                stream.write_all(&data);
            },
            Err(e) => println!("couldn't get a client : {e:?}")
        };
    }
    Ok(())
}
 
