use std::net::TcpStream;
use std::io::{Write};
use std::str;
use lib;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7000").unwrap();

    let req = lib::Request{
        message_body_size: 8,
        message_type: String::from("ECHO"),
        message_body: String::from("5"),
    };

    send_request(&stream, req);

    println!("Sent \"echo\", awaiting reply");

    read_response(&mut stream);

    println!("Response: {:#?}", str::from_utf8(&pong_response).unwrap());
}


fn send_request(stream: &TcpStream, req: lib::Request) {
    let buffer = req.to_buffer();

    stream.write(&buffer);
}