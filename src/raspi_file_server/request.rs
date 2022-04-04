use std::net::TcpStream;
use std::error::Error;
use std::fmt::Display;

pub struct Request {
    raw_content: String,
    path: String,
    method: HttpMethod,
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

    pub fn method(&self) -> &HttpMethod {
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

        let mut words = raw_content.split(" ");
        let (method, path) = words.next().and_then(|m| {
            let m: Result<_, _> = m.try_into();
            Option::zip(m.ok(), words.next())
        }).and_then(|(m, p)| {
            Some((m, p.to_string()))
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    UPDATE,
    DELETE,
    PATCH
}

impl TryFrom<&str> for HttpMethod {
    type Error = RequestParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match &s.to_ascii_uppercase()[..] {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "UPDATE" => Ok(HttpMethod::UPDATE),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(RequestParseError)
        }
    }
}

impl std::cmp::PartialEq<&str> for HttpMethod {
    fn eq(&self, other: &&str) -> bool {
        match (self, &other.to_ascii_uppercase()[..]) {
            (HttpMethod::GET, "GET") |
            (HttpMethod::POST, "POST") |
            (HttpMethod::PUT, "PUT") |
            (HttpMethod::UPDATE, "UPDATE") |
            (HttpMethod::DELETE, "DELETE") |
            (HttpMethod::PATCH, "PATCH") => true, 
            _ => false
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_http_method_part_eq_with_str() {
        assert_eq!(HttpMethod::GET, "GET");
        assert_eq!(HttpMethod::PUT, "put");
        assert_ne!(HttpMethod::PATCH, "GET");
    }

    #[test]
    pub fn test_http_method_from_str() {
        assert!(<&str as TryInto<HttpMethod>>::try_into("GET").is_ok());
        assert!(<&str as TryInto<HttpMethod>>::try_into("dElEtE").is_ok());
        assert!(<&str as TryInto<HttpMethod>>::try_into("asdf").is_err());
        assert_eq!(<&str as TryInto<HttpMethod>>::try_into("PATCH").unwrap(), HttpMethod::PATCH);
    }
}