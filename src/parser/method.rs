use crate::types::method::Method;

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