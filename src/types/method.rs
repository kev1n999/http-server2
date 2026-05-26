// Available http methods
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete, 
    Invalid,
}