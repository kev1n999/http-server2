use crate::server::routes::routes::routes;
use crate::{headers::request_parse::parse_request};
use crate::types::{
    method_type::Method,
};

use std::{
    io::{BufReader, prelude::*},
    net::{TcpStream},
};

/// based on https://doc.rust-lang.org/book/ch21-01-single-threaded.html
// function to handle http connections and get the requests
pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if let Some(request) = parse_request(http_request) {
        for route in routes() {
            if request.method == Method::Get && request.path == "/".to_string() {
                (route.handler)(&request, &stream);
            }
        }
    }
}