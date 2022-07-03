use std::collections::HashMap;

use color_eyre::Result;

pub fn header_parser(raw_response: &str) -> Result<HashMap<String, String>> {
    // split by line and  remove empty lines
    let headers = raw_response
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    // skip first line
    let headers = &headers[1..];

    // parse headers
    let headers = headers
        .iter()
        .map(|s| {
            let mut s = s.splitn(2, ':');
            let key = s.next().unwrap().trim();
            let value = s.next().unwrap().trim();
            (key.to_string(), value.to_string())
        })
        .collect::<HashMap<_, _>>();

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_parser() {
        let raw_response =
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 12\r\n\r\n";
        let headers = header_parser(raw_response).unwrap();
        assert_eq!(headers.len(), 2);
        assert_eq!(headers["Content-Type"], "application/json");
        assert_eq!(headers["Content-Length"], "12");
    }
}
