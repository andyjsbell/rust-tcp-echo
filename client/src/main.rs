// Our CLI to connect to our server on port 7000.  We read the console for text to send.
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    println!("Welcome, please enter your message to send");

    match TcpStream::connect("127.0.0.1:7000") {
        Ok(mut stream) => {
            let mut message_to_send = String::new();
            while match std::io::stdin().read_line(&mut message_to_send) {
                Ok(size) => {
                    print!("Sending message: {}", message_to_send);
                    println!("Size: {}", size);
                    
                    let size_written = stream.write(message_to_send.as_bytes()).unwrap();
                    message_to_send.clear();
                    
                    println!("Size written to socket: {}", size_written);
                    let mut data: [u8; 32] = [0;32];
                    match stream.read(&mut data) {
                        Ok(size) => {
                            println!("Read back {} bytes", size);
                            println!("Server replied: {}", from_utf8(&data).unwrap());
                            true
                        }
                        Err(_) => {
                            println!("Error reading back response from server");
                            false
                        }
                    }
                }
                Err(_) => {
                    println!("Error");
                    false
                }    
            } {}            
        }
        Err(_) => {
            println!("Error connecting to server");
        }
    }    
}
