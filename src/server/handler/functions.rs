use crate::{parser::json_parse::parse_json, types::{
    content_type::ContentType, request_type::Request, response_type::Response, status_code::StatusCode
}};

use std::net::TcpStream;
use serde::Deserialize;

pub fn home(request: &Request, stream: &TcpStream) -> Result<(), std::io::Error> {
    let response = Response::build_response(&StatusCode::Ok, ContentType::Html, "<h1>Hello World!</h1>");
    response.send(stream)
}

pub fn test(request: &Request, stream: &TcpStream) -> Result<(), std::io::Error> {
    let response = Response::build_response(&StatusCode::Ok, ContentType::Json, r#"{ "message": "Hello World in json!" }"#);
    response.send(stream)
}

pub fn sum_two_numbers(request: &Request, stream: &TcpStream) -> Result<(), std::io::Error> {
    let Request { body, .. } = request;
    
    #[derive(Deserialize)]
    struct Sum {
        x: i32,
        y: i32,
    }
    
    let json: Sum = parse_json(body)?;
    let sum = json.x + json.y;

    let response = Response::build_response(&StatusCode::Ok, ContentType::Text, &format!("the sum is: {}", sum));
    response.send(stream)
}