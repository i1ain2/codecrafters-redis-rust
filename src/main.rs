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
        let req_string = String::from_utf8(buf[..size].to_vec()).unwrap();
        let resp_array: Vec<&str> = req_string.split("\r\n").collect();
        let cmd = resp_array.get(2).unwrap().to_uppercase();
        match &*cmd {
            "PING" => {
                stream.write(b"+PONG\r\n")?;
            }
            "ECHO" => {
                // https://redis.io/docs/reference/protocol-spec/#resp-bulk-strings
                let s = resp_array.get(4).unwrap();
                let echo = format!("${}\r\n{}\r\n", s.len(), s);
                stream.write(echo.as_bytes())?;
            }
            _ => println!("not matched: {:?}", resp_array),
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
