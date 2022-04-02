use std::net::TcpStream;
use std::error::Error;
use std::fmt::Display;

pub struct Request {
    raw_content: String,
    path: String,
    method: String,
    params: Vec<(String, Option<String>)>,
}

#[allow(dead_code)]
impl Request {
    pub fn raw_content(&self) -> &String {
        &self.raw_content
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn params(&self) -> &Vec<(String, Option<String>)> {
        &self.params
    }
}

impl TryFrom<&TcpStream> for Request {
    type Error = Box<dyn Error>;
    fn try_from(stream: &TcpStream) -> Result<Self, Self::Error> {
        let mut buffer = Box::new([0u8;5120]);
        stream.peek(&mut *buffer)?;
        let raw_content = String::from_utf8_lossy(&*buffer).to_string();

        let mut method = String::new();
        let mut path = String::new();

        let mut words = raw_content.split(" ");
        words.next().and_then(|m| {
            method = m.to_owned();
            words.next()
        }).and_then(|p| {
            path = p.to_owned();
            Some(())
        }).ok_or(RequestParseError)?;

        let params: Vec<(String, Option<String>)> = path
            .split(|c| c == '?' || c == '&')
            .skip(1)
            .map(|s| {
                let mut key_val = s.split('=').map(ToString::to_string);
                (key_val.next(), key_val.next())
            })
            .filter(|(key, _)| key.is_some())
            .map(|(key, val)| (key.unwrap(), val))
            .collect();

        Ok(Request {
            raw_content,
            path,
            method,
            params,
        })
    }
}

#[derive(Debug)]
pub struct RequestParseError;

impl Display for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestParseError")
    }
}

impl Error for RequestParseError {}
