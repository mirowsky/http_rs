pub struct HttpStatusLine {
    // version of http protocol used.
    pub version: String,
    // status code of the response.
    pub status_code: u16,
    // reason phrase of the response.
    pub reason_phrase: String,
}

pub fn parse_status_line(status_line: &str) -> Result<HttpStatusLine> {
    unimplemented!()
}
