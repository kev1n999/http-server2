use crate::types::{
    response::Response,
    request::Request,
    method::Method,
};

use std::collections::HashMap;

pub fn parse_request(lines: Vec<String>) -> Option<Request> {
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut iter = lines.into_iter();  
    let request_line = iter.next().unwrap();
    let mut req_line_parts = request_line.split_whitespace(); // [METHOD, PATH, VERSION]
    let method = req_line_parts.next().unwrap().to_string();
    let path = req_line_parts.next().unwrap().to_string();
    let version = req_line_parts.next().unwrap().to_string();

    for header_lines in iter {
        match header_lines.split_once(':') {
            Some((key, val)) => {
                headers.insert(key.trim().to_string(), val.trim().to_string());
            },
            None => eprintln!("An error ocurred to parse the request header!"),
        }
    }

    Some(Request {
        version,
        path,
        method: Method::new(method),
        headers,
    })
}