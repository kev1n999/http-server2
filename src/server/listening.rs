use std::{
    io::{BufReader, prelude::*},
    net::{TcpStream},
};

use crate::headers::request_parse::parse_request;

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    parse_request(http_request);
}