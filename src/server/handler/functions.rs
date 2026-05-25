use crate::{parser::{file_parse::parse_static_file, json_parse::parse_json}, types::{
    content_type::ContentType, request_type::Request, response_type::Response, status_code::StatusCode
}};

use std::net::TcpStream;
use serde::Deserialize;

pub fn home(request: &Request, stream: &TcpStream) -> Result<(), std::io::Error> {
    let home_file = parse_static_file("home.html")?;
    let response = Response::build_response(&StatusCode::Ok, ContentType::Html, &home_file);
    response.send(stream)
}

pub fn calc(request: &Request, stream: &TcpStream) -> Result<(), std::io::Error> {
    let calc_file = parse_static_file("calc.html")?;
    let response = Response::build_response(&StatusCode::Ok, ContentType::Html, &calc_file);
    response.send(stream)
}

pub fn sum(request: &Request, stream: &TcpStream) -> Result<(), std::io::Error> {
    let Request { body, .. } = request;

    #[derive(Deserialize)]
    struct Sum {
        x: i32,
        y: i32,
    }
    
    let json: Sum = parse_json(body)?;
    let sum = json.x + json.y;

    let response = Response::build_response(&StatusCode::Ok, ContentType::Text, &format!("{}", sum));
    response.send(stream)
}