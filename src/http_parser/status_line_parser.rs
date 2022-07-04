use color_eyre::{Report, Result};

pub struct HttpStatusLine {
    // version of http protocol used.
    pub version: String,
    // status code of the response.
    pub status_code: u16,
    // reason phrase of the response.
    pub reason_phrase: String,
}

pub fn parse_status_line(raw_response: &str) -> Result<HttpStatusLine> {
    let mut status_line = raw_response
        .lines()
        .next()
        .ok_or_else(|| Report::msg("No status line"))?
        .split_whitespace();

    Ok(HttpStatusLine {
        version: status_line
            .next()
            .ok_or_else(|| Report::msg("No version"))?
            .to_string(),
        status_code: status_line
            .next()
            .ok_or_else(|| Report::msg("No status code"))?
            .parse()
            .map_err(|_| Report::msg("Could not parse status code"))?,
        reason_phrase: status_line
            .next()
            .ok_or_else(|| Report::msg("No reason phrase"))?
            .to_string(),
    })
}

#[cfg(test)]
mod tests {

    use crate::http_parser::mocks::response::RAW_RESPONSE_MOCK;

    use super::*;
    #[test]
    fn test_parse_status_line() {
        let status_line = parse_status_line(RAW_RESPONSE_MOCK).unwrap();

        assert_eq!(status_line.version, "HTTP/1.1");
        assert_eq!(status_line.status_code, 200);
        assert_eq!(status_line.reason_phrase, "OK");
    }
}
