use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
 
    println!("Request: {:#?}", http_request);
}
