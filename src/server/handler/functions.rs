use crate::types::{
    request_type::Request,
    response_type::Response,
    status_code::StatusCode,
    content_type::ContentType,
};

use std::net::TcpStream;

pub fn home(request: &Request, stream: &TcpStream) -> Result<(), std::io::Error> {
    let response = Response::build_response(&StatusCode::Ok, ContentType::Html, "<h1>Hello World!</h1>");
    response.send(stream)
}