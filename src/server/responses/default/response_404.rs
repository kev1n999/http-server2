use crate::{parser::{r#static::parse_static_file}, types::{
    server::{response::Response},
    headers::{content_type::ContentType, status_code::StatusCode},
}};

use std::net::TcpStream;

pub fn response_404(stream: &TcpStream) -> Result<(), std::io::Error> {
    let content = parse_static_file("404.html")?;
    let response = Response::new(
        &StatusCode::NotFound,
        ContentType::Html,
        content.as_str(),
    );

    response.send(stream)
}