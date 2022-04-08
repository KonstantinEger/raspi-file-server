use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::response::{Response, HttpStatusCode};

/// A (non-exhaustive) list of HTTP method types
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    PUT,
    PATCH,
    DELETE
}

impl TryFrom<&str> for HttpMethod {
    type Error = RequestParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpMethod::GET),
            "PUT" => Ok(HttpMethod::PUT),
            "PATCH" => Ok(HttpMethod::PATCH),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(RequestParseError)
        }
    }
}

/// An object representing a HTTP request.
///
/// Through the request struct, the raw content of the HTTP
/// request can be accessed, as well as the full [path](Request::path_as_str),
/// the [method](Request::method), query parameters with [Request::queries] or
/// url parameters with [Request::params] (not yet implemented).
#[derive(Debug)]
pub struct Request {
    raw_content: String,
    path: String,
    method: HttpMethod,
    queries: HashMap<String, Option<String>>,
}

impl Request {
    /// Returns the full raw content of the request in form of the string
    /// in which it was sent to the server.
    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    /// Returns the original full path with which the request was sent.
    pub fn path_as_str(&self) -> &str {
        &self.path
    }

    /// Returns the [HttpMethod] with which the request was sent.
    pub fn method(&self) -> HttpMethod {
        self.method
    }

    /// Returns a reference to a [HashMap] containing the encoded query parameters.
    ///
    /// Parameters are encoded in the path of the request. Query parameters
    /// consist of a key and an optional value. The first query parameter is prefixed
    /// with a `?`, following ones are separated by `&`. See the example below.
    /// ```
    /// use raspi_file_server::*;
    ///
    /// fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    ///     Server::new()
    ///         .add_route(HttpMethod::GET, "/greet", greet_route)
    ///         .bind_and_run("127.0.0.1:8080")?;
    ///     Ok(())
    /// }
    ///
    /// // matches requests like /greet?name=johndoe&otherQuery
    /// fn greet_route(req: &Request) -> Response {
    ///     if let Some(Some(name)) = req.queries().get("name") {
    ///         if req.queries().get("otherQuery").is_some() {
    ///             format!("Hello {}, I see you set the other query parameter ;)", name).into()
    ///         } else {
    ///             format!("Hello {}, nice to meet you!", name).into()
    ///         }
    ///     } else {
    ///         let mut response: Response = "unable to get name parameter".into();
    ///         response.set_status_code(HttpStatusCode::BadRequest);
    ///         response
    ///     }
    /// }
    /// ```
    /// A request to `/greet?name=johndoe` would return the response
    /// `Hello johndoe, nice to meet you!`, while a request to `/greet/name=johndoe?otherQuery`
    /// would yield `Hello johndoe, I see you set the other query parameter ;)`. A request
    /// where `name=...` is not present or hasn't set a value, the `BadRequest` response
    /// is sent.
    pub fn queries(&self) -> &HashMap<String, Option<String>> {
        &self.queries
    }

    /// Returns a reference to a [HashMap] containing the encoded url parameters.
    ///
    /// Parameters are encoded as elements of the path of the request, e.g.
    /// the [route](crate::Server::add_route) `/greet/{name}` activated by a
    /// request to `/greet/johndoe` would get passed a Request object where
    /// `request.params().get("name")` yields a Some("johndoe") value.
    /// ```
    /// use raspi_file_server::*;
    ///
    /// fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    ///     Server::new()
    ///         .add_route(HttpMethod::GET, "/greet/{name}", greet_route)
    ///         .bind_and_run("127.0.0.1:8080")?;
    ///     Ok(())
    /// }
    ///
    /// fn greet_route(req: &Request) -> Response {
    ///     if let Some(name) = req.params().get("name") {
    ///         format!("Hello {}, nice to meet you!", name).into()
    ///     } else {
    ///         let mut response: Response = "unable to get name parameter".into();
    ///         response.set_status_code(HttpStatusCode::BadRequest);
    ///         response
    ///     }
    /// }
    /// ```
    pub fn params(&self) -> &HashMap<String, String> {
        todo!()
    }
}

pub fn parse_request_from_http_request_body(content: String) -> Result<Request, RequestParseError> {
    let (method, path) = {
        let mut words = content.split(' ');
        words.next().ok_or(RequestParseError)
            .and_then(HttpMethod::try_from)
            .map(|m| (m, words.next().ok_or(RequestParseError)))
            .and_then(|(m, pr)| Ok((m, pr?)))
            .map(|(m, p)| (m, p.to_string()))?
    };

    let queries = path
        .split(|c| c == '?' || c == '&')
        .skip(1)
        .map(|keyval| {
            let mut keyval = keyval.split('=').map(ToString::to_string);
            (keyval.next(), keyval.next())
        })
        .filter(|(key, _)| key.is_some())
        .map(|(key, val)| (key.unwrap(), val))
        .collect();

    Ok(Request {
        raw_content: content,
        path,
        method,
        queries,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RequestParseError;

impl Into<Response> for RequestParseError {
    fn into(self) -> Response {
        let mut resp = Response::default();
        resp.set_html("RequestParseError");
        resp.set_status_code(HttpStatusCode::BadRequest);
        resp
    }
}

impl Display for RequestParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestParseError")
    }
}

impl std::error::Error for RequestParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_request() {
        let request_str = r"GET /path?query2&query1=val HTTP/1.1
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
Host: www.loremipsum.com
Accept-Language: en-us
Accept-Encoding: gzip, deflate
Connection: Keep-Alive".to_string();

        let request = parse_request_from_http_request_body(request_str.clone()).unwrap();
        assert_eq!(request.raw_content, request_str);
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.queries.len(), 2);
        assert_eq!(*request.queries.get("query1").unwrap(), Some("val".to_string()));
        assert_eq!(*request.queries.get("query2").unwrap(), None);
    }
}
