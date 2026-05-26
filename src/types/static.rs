use crate::types::content_type::{ContentType};
use crate::parser::r#static::parse_static_file;
use crate::types::response::Response;
use crate::types::status_code::StatusCode;

use std::net::TcpStream;

#[derive(Debug)]
pub struct StaticFile {
    pub path: String,
    pub content_type: ContentType,
}

impl StaticFile {
    pub fn new(path: String, content_type: ContentType) -> Self {
        Self { path, content_type }
    }
    pub fn send(&self, stream: &TcpStream) -> Result<(), std::io::Error> {
        let content = parse_static_file(&self.path)?;
        let response = Response::build_response(&StatusCode::Ok, self.content_type, &content);
        response.send(stream)
    }
}