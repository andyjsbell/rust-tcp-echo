// Our server that opens a TCP socket on port 7000 that echoes back what it receives
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Shutdown;
use std::io::Read;
use std::io::Write;

fn new_client(mut stream:TcpStream) {
    println!("Client connected {}", stream.peer_addr().unwrap());
    let mut data: [u8; 32] = [0; 32];  // Read in blocks of 32 bytes
    while 
        match stream.read(&mut data) {
            Ok(size) => {
                println!("Received {} bytes and writing back to client", size);
                stream.write(&data[0..size]);
                // Keep the while loop running
                true
            }
            Err(_) => {
                println!("Error!");
                stream.shutdown(Shutdown::Both).expect("Should shutdown");
                // Break out
                false
            }
        } {} // <-- This is the while loop here, we use match to return false/true for while, nice
}

fn new_client_and_close(mut stream: TcpStream) {
    println!("Client connected {}", stream.peer_addr().unwrap());
    let mut data: [u8; 32] = [0; 32];

    let size_read = stream.read(&mut data).unwrap();
    println!("Size read: {}", size_read);
    stream.write(&data);
}

pub fn run() {
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
