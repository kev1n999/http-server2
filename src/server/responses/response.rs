use std::collections::HashMap;
use std::{
    io::{Write},
    net::{TcpStream},
};

use crate::types::{
    server::response::Response,
    headers::content_type::ContentType,
    headers::status_code::StatusCode,
};

impl Response {
    pub fn send(&self, mut stream: &TcpStream) -> Result<(), std::io::Error> {
        stream.write_all(&self.to_bytes())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new(); // byte buffer 
        let response_line = format!("{} {} {}\r\n", self.version, self.status_code, self.reason);
        response.extend_from_slice(response_line.as_bytes());

        for (key, val) in &self.headers {
            let header = format!("{}: {}\r\n", key, val);
            response.extend_from_slice(header.as_bytes());
        }

        response.extend_from_slice("\r\n".as_bytes());
        response.extend_from_slice(&self.body);
        response
    }
    pub fn build_response(status_code: &StatusCode, content_type: ContentType, body: &str) -> Self {
        let headers = Self::headers_build(content_type, body.len()).expect("An error ocurred to build the headers!");

        Self {
            version: "HTTP/1.1".to_string(), 
            status_code: status_code.code(),
            reason: status_code.reason().to_string(),
            headers,
            body: body.as_bytes().to_vec(),
        }
    }
    pub fn headers_build(content_type: ContentType, body_len: usize) -> Result<HashMap<String, String>, String> {
        let mut headers_hash: HashMap<String, String> = HashMap::new(); // headers of server response
        let content_type = ContentType::to_str(&content_type); // convert content-type to &str to send in the headers
        let charset = "charset=utf-8"; 

        headers_hash.insert("Content-Type".to_string(), format!("{}; {}", content_type, charset));
        headers_hash.insert("Content-Length".to_string(),body_len.to_string());
        Ok(headers_hash)
    }
}