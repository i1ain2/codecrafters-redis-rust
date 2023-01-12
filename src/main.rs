use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: &TcpStream) -> std::io::Result<()> {
    loop {
        let mut buf = [0; 512];
        let size = stream.read(&mut buf)?;
        if size == 0 {
            break;
        }
        let req = String::from_utf8(buf[..size].to_vec()).unwrap();
        match &*req {
            // "PING" => {
            //     stream.write(b"+PONG\r\n")?;
            // }
            // _ => passprintln!("not matched: {}", req),
            _ => {
                stream.write(b"+PONG\r\n")?;
                stream.flush()?;
            }
        }
    }
    Ok(())
}

fn main() {
    let listerner = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listerner.incoming() {
        match stream {
            Ok(mut _stream) => {
                let _ = handle_client(&_stream);
            }
            Err(e) => {
                panic!("ecountered IO error: {e}")
            }
        }
    }
}
