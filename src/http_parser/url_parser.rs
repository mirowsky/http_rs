use color_eyre::Result;

pub struct HttpUrl {
    // tells the server which protocol to use for the connection.
    pub scheme: String,
    // tells the server which host to connect to.
    pub host: String,
    // tells the server which port to connect to.
    pub port: u16,
    // tells the server which path to request.
    pub path: String,
    // tells the server which query string to request.
    pub query: Option<String>,
    // indicates a specific location within a html document such as an id attribute.
    pub fragment: Option<String>,
}

pub fn parse_url(url: &str) -> Result<HttpUrl> {
    unimplemented!()
}
