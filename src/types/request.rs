use std::collections::HashMap; 

use crate::types::method;

#[derive(Debug)]
pub struct Request {
    pub version: String,
    pub path: String,
    pub method: method::Method,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}