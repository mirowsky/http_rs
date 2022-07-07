#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Copy, Clone, Eq, Ord, Hash, PartialEq, PartialOrd)]
enum HttpVersion {
    Http09,
    Http10,
    Http11,
    Http20,
    Http30,
    NotHttp,
}

impl Default for HttpVersion {
    fn default() -> Self {
        HttpVersion::HTTP_20
    }
}

impl Debug for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http09 => write!(f, "Http09"),
            Self::Http10 => write!(f, "Http10"),
            Self::Http11 => write!(f, "Http11"),
            Self::Http20 => write!(f, "Http20"),
            Self::Http30 => write!(f, "Http30"),
            Self::NotHttp => {
                panic!("HttpVersion::NotHttp is not a valid HttpVersion");
            }
        }
    }
}

impl HttpVersion {
    // HTTP/0.9
    pub const HTTP_09: HttpVersion = HttpVersion::Http09;

    // HTTP/1.0
    pub const HTTP_10: HttpVersion = HttpVersion::Http10;

    // HTTP/1.1
    pub const HTTP_11: HttpVersion = HttpVersion::Http11;

    // HTTP/2.0
    pub const HTTP_20: HttpVersion = HttpVersion::Http20;

    // HTTP/3.0
    pub const HTTP_30: HttpVersion = HttpVersion::Http30;
}
