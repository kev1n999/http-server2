use serde::de::DeserializeOwned;

pub fn parse_json<S>(request_json: &[u8]) -> Result<S, serde_json::Error>
    where S: DeserializeOwned
{   
    serde_json::from_slice(request_json)
}