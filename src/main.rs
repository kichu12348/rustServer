use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");

    println!("Server running on 127.0.0.1:7878");

    // Listen for incoming connections
    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");

        // Read data from the stream
        let mut buffer = [0; 512];
        stream.read(&mut buffer).expect("Failed to read from stream");

        // Print the received message
        println!("Received: {}", String::from_utf8_lossy(&buffer[..]));

        // Send a response back to the client
        let response = "Hello from the server!";
        stream.write(response.as_bytes()).expect("Failed to write to stream");
        stream.flush().expect("Failed to flush stream");
    }
}