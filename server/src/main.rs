// Our server that opens a TCP socket on port 7000 that echoes back what it receives
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

fn new_client_and_close(mut stream: TcpStream) {
    println!("Client connected {}", stream.peer_addr().unwrap());
    let mut data: [u8; 32] = [0; 32];

    let size_read = stream.read(&mut data).unwrap();
    println!("Size read: {}", size_read);
    stream.write(&data);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                new_client(stream);
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }

    drop(listener);
}
