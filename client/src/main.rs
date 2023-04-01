use std::net::TcpStream;
use std::io::{Write, BufReader};
use std::io::prelude::*;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7000").unwrap();

    stream.write(b"Hello world").unwrap();
    println!("Sent \"Hello world\", awaiting reply");

    let buf_reader = BufReader::new(&mut stream);

    let response: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", response);
}
