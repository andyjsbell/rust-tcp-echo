// Our CLI to connect to our server on port 7000.  We read the console for text to send.
use std::net::{TcpStream};
use std::io::{Read, Write};

fn main() {
    println!("Welcome, please enter your message to send");

    let mut message_to_send = String::new();
    while match std::io::stdin().read_line(&mut message_to_send) {
        Ok(size) => {
            print!("Sending message: {}", message_to_send);
            println!("Size: {}", size);
            let connect = TcpStream::connect("127.0.0.1:7000");
            match connect {
                Ok(mut stream) => {
                    let size_written = stream.write(message_to_send.as_bytes()).unwrap();
                    println!("Size written to socket: {}", size_written);
                    true
                }
                Err(_) => {
                    println!("Error connecting to server");
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
