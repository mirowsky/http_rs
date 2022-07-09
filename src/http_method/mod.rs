#[derive(Clone, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    Custom(String),
}
