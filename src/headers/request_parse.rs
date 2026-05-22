use crate::types::{
    response::Response,
    request::Request,
};

// pub struct Request {
//     pub version: String,
//     pub path: String,
//     pub method: method::Method,
//     pub headers: HashMap<String, String>,
//     pub body: Vec<u8>,
// }

pub fn parse_request(lines: Vec<String>) -> () {
    let mut iter = lines.into_iter();  
    let request_line = iter.next().unwrap();
    let mut req_line_parts = request_line.split_whitespace(); // [METHOD, PATH, VERSION]
    let method = req_line_parts.next();
    let path = req_line_parts.next();
    let version = req_line_parts.next();

    println!("Request Line:\nMETHOD: {:?}\nPATH: {:?}\nVERSION: {:?}", method, path, version);
}