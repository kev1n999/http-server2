use std::collections::HashMap;

use crate::types::{
    response_type::Response,
    content_type::ContentType,
    status_code::StatusCode,
};

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();
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
    pub fn build_response(status_code: &StatusCode, content_type: ContentType, body: Vec<u8>) -> Self {
        let headers = Self::headers_build(content_type).expect("An error ocurred to build the headers!");

        Self {
            version: "HTTP/1.1".to_string(),
            status_code: status_code.code(),
            reason: status_code.reason().to_string(),
            headers,
            body,
        }
    }
    pub fn headers_build(content_type: ContentType) -> Result<HashMap<String, String>, String> {
        let mut headers_hash: HashMap<String, String> = HashMap::new();
        let content_type = ContentType::to_str(&content_type);

        headers_hash.insert("Content-Type".to_string(), content_type.to_string());
        // headers_hash.insert("Content-Length".to_string(), self.body.len().to_string());
        Ok(headers_hash)
    }
}