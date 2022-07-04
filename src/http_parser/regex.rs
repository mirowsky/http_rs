use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref BODY_REGEX: Regex = Regex::new(r"(?im)^\s*\{{1}[\s\S]*\}{1}$").unwrap();
    pub static ref HEADER_REGEX: Regex = Regex::new(r"(?im)^[^\s\W].*:{1}.*$").unwrap();
}
