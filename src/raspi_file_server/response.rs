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
    status: StatusCode,
}

#[allow(dead_code)]
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

    pub fn set_status(mut self, code: StatusCode) -> Self {
        self.status = code;
        self
    }

    pub fn send(mut self) -> Result<(), Box<dyn Error>> {
        let temp = format!(
            "HTTP/1.1 {}\n{}\nContent-Length: {}\n\n{}",
            format!(
                "{} {:?}",
                <StatusCode as Into<usize>>::into(self.status),
                self.status
            ),
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
            status: StatusCode::OK,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    OK,                  // 200
    Created,             // 201
    Accepted,            // 202
    BadRequest,          // 400
    Unauthorized,        // 401
    Forbidden,           // 403
    NotFound,            // 404
    InternalServerError, // 500
    NotImplemented,      // 501
}

impl From<usize> for StatusCode {
    fn from(code: usize) -> StatusCode {
        use StatusCode::*;
        match code {
            200 => OK,
            201 => Created,
            202 => Accepted,
            400 => BadRequest,
            401 => Unauthorized,
            403 => Forbidden,
            404 => NotFound,
            500 => InternalServerError,
            501 => NotImplemented,
            _ => panic!("unknown status code")
        }
    }
}

impl From<StatusCode> for usize {
    fn from(code: StatusCode) -> Self {
        use StatusCode::*;
        match code {
            OK => 200,
            Created => 201,
            Accepted => 202,
            BadRequest => 400,
            Unauthorized => 401,
            Forbidden => 403,
            NotFound => 404,
            InternalServerError => 500,
            NotImplemented => 501
        }
    }
}
