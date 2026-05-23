#[derive(Debug)]
pub enum ContentType {
    Text,
    Html,
    Json,
    JavaScript,
}

impl ContentType {
    pub fn from_str(content_type: &str) -> Option<Self> {
        match content_type.trim().to_lowercase().as_str() {
            "text/plain" => Some(ContentType::Text),
            "text/html" => Some(ContentType::Html),
            "text/javascript" => Some(ContentType::JavaScript),
            "application/json" => Some(ContentType::Json),
            _ => None
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            ContentType::Text => "text/plain",
            ContentType::Html => "text/html",
            ContentType::JavaScript => "text/javascript",
            ContentType::Json => "application/json",
        }
    }
}