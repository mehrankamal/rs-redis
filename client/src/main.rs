use std::net::TcpStream;
use std::io::{Write, BufReader};
use std::io::prelude::*;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7000").unwrap();

    let ping_message = b"ping";
    stream.write(ping_message).unwrap();

    println!("Sent \"ping\", awaiting reply");

    let mut buf_reader = BufReader::new(&mut stream);

    let mut pong_response: [u8; 4] = [0; 4];
    buf_reader.read(&mut pong_response).unwrap();

    println!("Response: {:#?}", str::from_utf8(&pong_response).unwrap());
}
