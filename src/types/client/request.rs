use std::collections::HashMap; 

use crate::types::headers::method::Method;

// Http Request Structure to parse
#[derive(Debug, PartialEq)]
pub struct Request {
    pub version: String,
    pub path: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}