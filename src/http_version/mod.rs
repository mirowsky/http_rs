#[derive(Copy, Clone, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub enum HttpVersion {
    Http09,
    Http10,
    Http11,
    Http20,
    Http30,
    NotHttp,
}
