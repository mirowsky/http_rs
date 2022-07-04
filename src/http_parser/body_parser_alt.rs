use std::collections::HashMap;

use color_eyre::Result;

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

#[cfg(test)]
mod tests {
    use crate::http_parser::mocks::response::RAW_RESPONSE_MOCK;

    use super::*;
    #[test]
    fn test_body_parser_alt() {
        let body = body_parser_alt(RAW_RESPONSE_MOCK).unwrap();

        assert_eq!(body_parser_alt(RAW_RESPONSE_MOCK).unwrap().len(), 4);
        assert_eq!(
            body_parser_alt(RAW_RESPONSE_MOCK)
                .unwrap()
                .get("hello")
                .unwrap(),
            "world"
        );

        assert_eq!(body["foo"], "bar");

        assert_eq!(body["hello"], "world");

        assert_eq!(body["baz"], "qux")
    }
}
