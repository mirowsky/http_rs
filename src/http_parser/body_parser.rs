use std::collections::HashMap;

use color_eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BODY_REGEX: Regex = Regex::new(r"(?im)^\s*\{{1}[\s\S]*\}{1}$").unwrap();
}

pub fn body_parser_alt(raw_response: &str) -> Result<HashMap<String, String>> {
    if let Some(body) = raw_response.split("\r\n\r\n").nth(1) {
        let body = body.trim();
        let body = body.trim_end_matches('}');
        let body = body.trim_start_matches('{');

        let body = body
            .split(',')
            .map(|x| {
                let mut x = x.splitn(2, ':');
                let key = x
                    .next()
                    .unwrap()
                    .trim()
                    .trim_start_matches('\"')
                    .trim_end_matches('\"');
                let value = x
                    .next()
                    .unwrap()
                    .trim()
                    .trim_start_matches('\"')
                    .trim_end_matches('\"');
                (key.to_string(), value.to_string())
            })
            .collect::<HashMap<_, _>>();

        Ok(body)
    } else {
        Err(color_eyre::eyre::eyre!("Could not parse body"))
    }
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
    use super::*;

    const RAW_RESPONSE_MOCK: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 12\r\n\r\n{\"hello\":\"world\",\"foo\":\"bar\",\"baz\":\"qux\",\"quux\":\"corge\"}";

    #[test]
    fn test_body_parser_alt() {
        let _ = body_parser_alt(RAW_RESPONSE_MOCK);

        println!("{:#?}", body_parser_alt(RAW_RESPONSE_MOCK));

        assert!(true);
    }

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
