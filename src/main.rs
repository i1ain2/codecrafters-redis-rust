use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: &TcpStream) -> () {
    let _ = stream.write(b"+PONG\r\n");
}

fn main() {
    let listerner = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listerner.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_client(&_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
