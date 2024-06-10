use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:11451").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: std::net::TcpStream) {
    let buf_read = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_read
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
}
