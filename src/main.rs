use std::net::TcpListener;

fn main() {
    let listerner = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listerner.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
