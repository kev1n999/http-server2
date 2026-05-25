use crate::server::routes::routes::routes;
use crate::{parser::request_parse::parse_request};

use crate::types::{
    method_type::Method,
};

use std::{
    io::{Read},
    net::{TcpStream},
};

// function to handle http connections and get the requests
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 4064];
    let req_bytes_received = stream.read(&mut buffer).unwrap();
    let str_request = String::from_utf8_lossy(&buffer[..req_bytes_received]);
    let parts_request: Vec<String> = str_request.split("\r\n\r\n").map(|s| s.to_string()).collect();
    
    if let Some(request) = parse_request(parts_request) {
        for route in routes() {
            if request.method == Method::Get && request.path == route.path {
                if let Err(err) = (route.handler)(&request, &stream) {
                    eprintln!("{err}");
                }
                break;
            } else if request.method == Method::Post && request.path == route.path {
                if let Err(err) = (route.handler)(&request, &stream) {
                    eprintln!("{err}");
                }
                break;
            }
        }
    }
}