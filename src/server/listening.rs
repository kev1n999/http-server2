use crate::headers::request_parse::parse_request;
use crate::server::responses::html::hello_world;

use std::{
    io::{BufReader, prelude::*},
    net::{TcpStream},
};

/// based on https://doc.rust-lang.org/book/ch21-01-single-threaded.html
// function to handle http connections and get the requests
pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parsed = parse_request(http_request);
    println!("New request: {:#?}", parsed);
    hello_world(&stream);
}