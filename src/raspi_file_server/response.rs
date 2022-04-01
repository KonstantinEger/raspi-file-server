use std::net::TcpStream;
use std::error::Error;
use std::io::prelude::*;

use HeaderName::*;

#[derive(Clone, Copy)]
pub enum HeaderName {
    ContentType,
}

impl From<HeaderName> for &str {
    fn from(hn: HeaderName) -> Self {
        match hn {
            ContentType => "Content-Type"
        }
    }
}

pub struct Response {
    stream: TcpStream,
    body: String,
    headers: Vec<(HeaderName, String)>,
}

impl Response {
    pub fn json<S>(mut self, json: S) -> Self
    where
        S: ToString
    {
        self = self.set_header(ContentType, "application/json");
        self.body = json.to_string();
        self
    }

    pub fn html<S>(mut self, html: S) -> Self
    where
        S: ToString
    {
        self = self.set_header(ContentType, "text/html");
        self.body = html.to_string();
        self
    }

    pub fn set_header<S>(mut self, name: HeaderName, value: S) -> Self
    where
        S: ToString
    {
        self.headers.push((name, value.to_string()));
        self
    }

    pub fn send(mut self) -> Result<(), Box<dyn Error>> {
        let temp = format!(
            "HTTP/1.1 200 OK\n{}\nContent-Length: {}\n\n{}",
            self.headers_to_string(),
            self.body.len(),
            self.body
        );
        self.stream.write(temp.as_bytes())?;
        Ok(())
    }

    fn headers_to_string(&self) -> String {
        self.headers
            .iter()
            .map(|(hn, s)| {
                format!("{}: {}", <HeaderName as Into<&str>>::into(*hn), s)
            }).collect::<Vec<String>>()
            .join("\n")
    }
}

impl From<TcpStream> for Response {
    fn from(s: TcpStream) -> Self {
        Response {
            stream: s,
            body: String::new(),
            headers: Vec::new(),
        }
    }
}
