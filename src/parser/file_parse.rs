use std::fs;

pub fn parse_static_file(path: &str) -> std::io::Result<String>{
    let contents = fs::read_to_string(&format!("src/server/public/{}", path)).expect("Should have been able to read the file");
    Ok(contents)
}