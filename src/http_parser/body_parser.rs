use color_eyre::Result;
use std::collections::HashMap;

use crate::http_parser::regex::BODY_REGEX;

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
            (remove_non_alphanumeric(key), remove_non_alphanumeric(value))
        })
        .collect::<HashMap<_, _>>();

    println!("{:#?}", body);

    Ok(HashMap::new())
}

fn remove_non_alphanumeric(input: &str) -> String {
    input.chars().filter(|c| c.is_alphanumeric()).collect()
}

#[cfg(test)]
mod tests {
    use crate::http_parser::mocks::response::RAW_RESPONSE_MOCK;

    use super::*;

    #[test]
    fn test_remove_non_alphanumeric() {
        assert_eq!(remove_non_alphanumeric("Hello World"), "HelloWorld");

        assert_eq!(remove_non_alphanumeric(r"'Hello': 'World'"), "HelloWorld");
    }

    #[test]
    fn test_body_parser() {
        let body_map = body_parser(RAW_RESPONSE_MOCK).unwrap();
        assert_eq!(body_map.get("name").unwrap(), "John Doe");
        assert_eq!(body_map.get("age").unwrap(), "42");
        assert_eq!(body_map.get("address").unwrap(), "123 Main St");
    }
}
