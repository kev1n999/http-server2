use crate::types::method::Method;

use std::fmt;

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "Put"),
            Method::Patch => write!(f, "Patch"),
            Method::Delete => write!(f, "DELETE"),
            _ => write!(f, "Invalid"),
        }
    }
}

impl Method {
    // method to convert the string method of a request to the equivalent type
    pub fn new(method: String) -> Self {
        match method.trim().to_lowercase().as_str() {
            "get" => Method::Get,
            "post" => Method::Post,
            "put" => Method::Put,
            "patch" => Method::Patch,
            "delete" => Method::Delete,
            _ => Method::Invalid,             
        }
    }
}