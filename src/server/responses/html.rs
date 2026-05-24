use crate::types::{
    response_type::Response,
    status_code::StatusCode,
    content_type::ContentType,
};

use std::net::TcpStream;

pub fn hello_world(stream: &TcpStream) -> Result<(), std::io::Error> {
    let response = Response::build_response(&StatusCode::Ok, ContentType::Html, "<h1>Hello World!</h1>");
    response.send(&stream)
}