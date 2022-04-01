use std::net::TcpStream;
use std::error::Error;
use std::io::prelude::*;

pub struct Response {
    stream: TcpStream,
}

impl Response {
    pub fn send(mut self) -> Result<(), Box<dyn Error>> {
        let temp = format!("HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: 11\n\nHello World");
        self.stream.write(temp.as_bytes())?;
        Ok(())
    }
}

impl From<TcpStream> for Response {
    fn from(s: TcpStream) -> Self {
        Response { stream: s }
    }
}