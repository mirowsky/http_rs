#![allow(unused)]

use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

use color_eyre::Result;

use crate::http_response::response::HttpResponse;

pub struct HttpClient<T: AsRef<str>> {
    url: T,
}

impl<T: AsRef<str>> HttpClient<T> {
    pub fn new(url: T) -> Self {
        HttpClient { url }
    }

    pub fn get(&self) -> Result<HttpResponse> {
        let host = "abc";
        let path = "/hello/world";

        let mut stream = TcpStream::connect(host).expect("Could not connect to the server");

        let mut request = String::new();
        request.push_str(format!("GET {} HTTTP/1.1", path).as_str());
        request.push_str("\r\n");
        request.push_str(format!("Host: {}", host).as_str());
        request.push_str("\r\n");
        request.push_str("Connection: close");
        request.push_str("\r\n");
        request.push_str("\r\n");

        let request_as_bytes = request.as_bytes();

        stream
            .write_all(request_as_bytes)
            .expect("Could not write to the stream...");

        let mut buffer = String::new();

        stream
            .read_to_string(&mut buffer)
            .expect("Could not read from the stream");

        stream
            .shutdown(std::net::Shutdown::Both)
            .expect("Could not shutdown stream");

        Ok(HttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: "".to_string(),
        })
    }
}
