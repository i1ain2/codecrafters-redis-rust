use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
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
        let stream = stream.unwrap();
        thread::spawn(|| {
            let _ = handle_client(stream);
        });
    }
}
