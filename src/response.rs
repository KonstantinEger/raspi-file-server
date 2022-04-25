use std::collections::HashMap;

/// A (non-exhaustive) list of HTTP status codes according to [MDN](https://developer.mozilla.org/de/docs/Web/HTTP/Status)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HttpStatusCode {
    OK,                  // 200
    BadRequest,          // 400
    NotFound,            // 404
    InternalServerError, // 500
}

impl From<HttpStatusCode> for usize {
    fn from(code: HttpStatusCode) -> Self {
        match code {
            HttpStatusCode::OK => 200,
            HttpStatusCode::BadRequest => 400,
            HttpStatusCode::NotFound => 404,
            HttpStatusCode::InternalServerError => 500,
        }
    }
}

impl Default for HttpStatusCode {
    fn default() -> Self {
        HttpStatusCode::OK
    }
}

/// A (non-exhaustive) list of HTTP headers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HttpHeaderName {
    ContentType,
}

impl From<HttpHeaderName> for &str {
    fn from(name: HttpHeaderName) -> Self {
        match name {
            HttpHeaderName::ContentType => "content-type",
        }
    }
}

/// An object representing a HTTP response.
///
/// By default, the body is empty, no headers are set and the status code
/// is 200 OK. Using various methods, these can be changed. For example,
/// [Response::set_body] changes only the body. In contrast, [Response::set_json]
/// changes the body _and_ sets the header `content-type: application/json`.
#[derive(Default)]
pub struct Response {
    status_code: HttpStatusCode,
    body: String,
    headers: HashMap<HttpHeaderName, String>,
}

impl Response {
    /// Sets the HTTP status code
    pub fn set_status_code(&mut self, code: HttpStatusCode) {
        self.status_code = code;
    }

    /// Sets a specific header.
    ///
    /// If a header with the same [HttpHeaderName] is already set, it will get overwritten.
    pub fn set_header<S: ToString>(&mut self, header_name: HttpHeaderName, header_value: S) {
        self.headers.insert(header_name, header_value.to_string());
    }

    /// Sets the body and only the body of the response.
    pub fn set_body<S: ToString>(&mut self, body: S) {
        self.body = body.to_string();
    }

    /// Sets the body of the response and the header `content-type: application/json`.
    pub fn set_json<S: ToString>(&mut self, json: S) {
        self.set_header(HttpHeaderName::ContentType, "application/json");
        self.body = json.to_string();
    }

    /// Sets the body of the response and the header `content-type: text/html`.
    ///
    /// This method is also used by the implementation of [From<&str>] for Response.
    pub fn set_html<S: ToString>(&mut self, html: S) {
        self.set_header(HttpHeaderName::ContentType, "text/html");
        self.body = html.to_string();
    }

    fn headers_to_string(&self) -> String {
        self.headers
            .iter()
            .map(|(hn, value)| format!("{}: {}", <HttpHeaderName as Into<&str>>::into(*hn), value))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

/// Converts a Response to a String which can be written to the response
/// [TcpStream](std::net::TcpStream).
pub fn response_into_http_response_string(response: Response) -> String {
    format!(
        "HTTP/1.1 {} {:?}\n{}\ncontent-length: {}\n\n{}",
        <HttpStatusCode as Into<usize>>::into(response.status_code),
        response.status_code,
        response.headers_to_string(),
        response.body.len(),
        response.body
    )
}

impl From<&str> for Response {
    fn from(s: &str) -> Self {
        let mut r = Response::default();
        r.set_html(s);
        r
    }
}

impl From<String> for Response {
    fn from(s: String) -> Self {
        <Response as From<&str>>::from(&s)
    }
}

impl<T, E> From<Result<T, E>> for Response
where
    T: Into<Response>,
    E: Into<Response>,
{
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(t) => t.into(),
            Err(e) => e.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_response() {
        let response = Response::default();
        assert_eq!(response.status_code, HttpStatusCode::OK);
        assert_eq!(response.body, "");
        assert_eq!(response.headers.len(), 0);
    }

    #[test]
    fn test_set_status_code() {
        let mut response = Response::default();
        response.set_status_code(HttpStatusCode::OK);
        assert_eq!(response.status_code, HttpStatusCode::OK);
        response.set_status_code(HttpStatusCode::BadRequest);
        assert_eq!(response.status_code, HttpStatusCode::BadRequest);
    }

    #[test]
    fn test_set_header() {
        let mut response = Response::default();
        assert_eq!(response.headers.len(), 0);
        response.set_header(HttpHeaderName::ContentType, "test1");
        assert_eq!(response.headers.len(), 1);
        response.set_header(HttpHeaderName::ContentType, "test2");
        assert_eq!(response.headers.len(), 1);
        assert_eq!(
            response.headers.get(&HttpHeaderName::ContentType).unwrap(),
            "test2"
        );
    }

    #[test]
    fn test_set_body() {
        let mut response = Response::default();
        response.set_body("body");
        assert_eq!(response.body, "body");
        assert_eq!(response.headers.len(), 0);
    }

    #[test]
    fn test_set_json_and_html() {
        let mut response = Response::default();
        response.set_json("json");
        assert_eq!(response.body, "json");
        assert_eq!(response.headers.len(), 1);
        assert_eq!(
            response.headers.get(&HttpHeaderName::ContentType).unwrap(),
            "application/json"
        );
        response.set_html("html");
        assert_eq!(response.body, "html");
        assert_eq!(response.headers.len(), 1);
        assert_eq!(
            response.headers.get(&HttpHeaderName::ContentType).unwrap(),
            "text/html"
        );
    }

    #[test]
    fn test_into_http_response_string() {
        let mut response = Response::default();
        response.set_html("test");
        let should_be = "HTTP/1.1 200 OK\ncontent-type: text/html\ncontent-length: 4\n\ntest";
        assert_eq!(response_into_http_response_string(response), should_be);
    }

    #[test]
    fn test_response_from_str() {
        let response: Response = "test".into();
        assert_eq!(response.body, "test");
        assert_eq!(
            response.headers.get(&HttpHeaderName::ContentType).unwrap(),
            "text/html"
        );
        assert_eq!(response.status_code, HttpStatusCode::OK);
    }
}
