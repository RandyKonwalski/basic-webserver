use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

pub mod http;
use http::{HTTPResponse, HTTPResponseHeader, HTTPBody};

fn handle_client(mut stream: TcpStream) {
    // Create a buffer with the size of 1024 bytes
    let mut buffer = [0; 1024];
    // Read the stream and put it in the buffer
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    // Create the Response
    let mut header = HTTPResponseHeader::new();
    header.append("Server", "TEST/1.0");
    let body = HTTPBody::new(Some("<b>Hello World!</b>"));
    let response = HTTPResponse::new("HTTP/1.1", 200, header, body);
    match stream.write(response.build().as_bytes()){
        Ok(_buf) =>{
            
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    // Create the TcpListener and unwrap
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on 127.0.0.1:8080!");
    // Listening for incoming connections
    for stream in listener.incoming(){
        // Matching if stream is ok or errored
        match stream {
            Ok(stream) => {
                // Connection succeeded
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Spawning a thread to handle the client asychronosly
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                // Connection produced an error
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}