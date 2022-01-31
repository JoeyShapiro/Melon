use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Connteced to Server");

            let msg = b"Hello Python!";

            stream.write(msg).unwrap();
            println!("sent data awaiting reply...");

            let mut data = [ 0 as u8; 13]; // 6B buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("{}", text);
                },
                Err(e) => {
                    println!("failed: {}", e);
                }
            }
        },
        Err(e) => {
            println!("failed connect: {}", e);
        }
    }
    println!("Terminated.");
}