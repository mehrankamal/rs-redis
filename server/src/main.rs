use std::{net::{TcpListener, TcpStream}, io::{BufReader, Read, Write}};
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Incoming stream opened!");
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    
    let mut ping_buffer = [0u8; 4];
    buf_reader.read(&mut ping_buffer).unwrap();
 
    println!("Request: {:#?}", str::from_utf8(&ping_buffer).unwrap());

    let pong_buffer = b"pong";
    stream.write(pong_buffer).unwrap();

    println!("Sent \"pong\"");
}
