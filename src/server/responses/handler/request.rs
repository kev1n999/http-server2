use crate::types::client::request::Request;

use std::path::Path;

pub fn request_is_file(request: &Request) -> bool {
    let valid_extensions = ["html", "css", "js", "png", "jpg", "jpeg", "svg", "ico"];

    Path::new(&request.path)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| valid_extensions.contains(&ext))
}