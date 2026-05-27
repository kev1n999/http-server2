use crate::types::{
    client::request::Request,
    headers::method::Method,
};

use std::collections::HashMap;

use std::fmt;

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nVersion: {}", self.version)?;
        writeln!(f, "Path: {}", self.path)?;
        writeln!(f, "Method: {}", self.method)?;

        writeln!(f, "Headers:")?;
        
        for (key, val) in &self.headers {
            writeln!(f, "{}: {}", key, val)?;
        }

        writeln!(f, "Body:")?;

        match std::str::from_utf8(&self.body) {
            Ok(body) => writeln!(f, "{}", body)?,
            Err(_) => writeln!(f, "{:?}", self.body)?,
        }

        Ok(())
    }
}

// function to parse a http request
pub fn parse_request(lines: Vec<String>) -> Option<Request> {
    let mut headers: HashMap<String, String> = HashMap::new(); // hashmap to storage the parsed header 
    let mut body: Vec<String> = Vec::new(); // vec to storage the request body

    let mut iter = lines.into_iter(); // convert the request lines to an iterator 
    let request_line = iter.next().unwrap(); // get the request line(method path version)
    let mut req_line_parts = request_line.split_whitespace(); // split request line -> [METHOD, PATH, VERSION]

    let method = req_line_parts.next().unwrap().to_string(); // request method 
    let path = req_line_parts.next().unwrap().to_string(); // request path 
    let version = req_line_parts.next().unwrap().to_string(); // http version

    for header_lines in iter.clone() {
        match header_lines.split_once(':') {
            Some((key, val)) => {
                headers.insert(key.trim().to_string(), val.trim().to_string());
            },
            None => eprintln!("An error ocurred to parse the request header!"),
        }
    }
    
    for body_val in iter {
        body.push(body_val)
    }

    Some(Request {
        version,
        path,
        method: Method::new(method),
        headers,
        body: body.into_iter().flat_map(|s| s.into_bytes()).collect(),
    })
}