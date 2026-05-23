use crate::headers::request_parse::parse_request;
use crate::types::{
    response_type::Response,
    status_code::StatusCode,
    content_type::ContentType,
};

use std::{
    io::{BufReader, prelude::*},
    net::{TcpStream},
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parsed = parse_request(http_request);
    let response = Response::build_response(&StatusCode::Ok, ContentType::Text, "Hello world".as_bytes().to_vec());
    stream.write_all(&Response::to_bytes(&response));
}