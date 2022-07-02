use std::collections::HashMap;

use color_eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BODY_REGEX: Regex = Regex::new(r"(?im)^\s*\{{1}[\s\S]*\}{1}$").unwrap();
}

pub fn remove_ascii_characters(input: &str) -> String {
    input.chars().filter(|c| c.is_alphanumeric()).collect()
}

pub fn body_parser(raw_response: &str) -> Result<HashMap<String, String>> {
    let body = BODY_REGEX
        .find(raw_response)
        .unwrap()
        .as_str()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    let body = body[1..body.len() - 1].join("");

    let body = body
        .split(',')
        .map(|s| {
            let mut s = s.splitn(2, ':');
            let key = s.next().unwrap().trim();
            let value = s.next().unwrap().trim();
            (remove_ascii_characters(key), remove_ascii_characters(value))
        })
        .collect::<HashMap<_, _>>();

    println!("{:#?}", body);

    Ok(HashMap::new())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_parser() {
        let raw_response = r#"HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 12

{   "hello": "world",
    "name": "John Doe",
    "age": "42",
    "address": "123 Main St"
}"#;

        let body_map = body_parser(raw_response).unwrap();
        assert_eq!(body_map.get("name").unwrap(), "John Doe");
        assert_eq!(body_map.get("age").unwrap(), "42");
        assert_eq!(body_map.get("address").unwrap(), "123 Main St");
    }
}
