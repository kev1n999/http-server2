use crate::{parser::file_parse::parse_static_file, types::{content_type::ContentType, response_type::Response, status_code::StatusCode}};

use std::net::TcpStream;

pub fn response_404(stream: &TcpStream) -> Result<(), std::io::Error> {
    let content = parse_static_file("404.html")?;
    let response = Response::build_response(
        &StatusCode::NotFound,
        ContentType::Html,
        content.as_str(),
    );

    response.send(stream)
}