use std::collections::HashMap;

/// A (non-exhaustive) list of HTTP status codes according to [MDN](https://developer.mozilla.org/de/docs/Web/HTTP/Status)
#[derive(Clone, Copy, Debug)]
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
    ContentType
}

impl From<HttpHeaderName> for &str {
    fn from(name: HttpHeaderName) -> Self {
        match name {
            HttpHeaderName::ContentType => "content-type"
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
    pub fn set_header<S: ToString>(&mut self, header_name: HttpHeaderName, header_value: S)
    {
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

    /// Converts a Response to a String which can be written to the response
    /// [TcpStream](std::net::TcpStream). This method is used by the library and
    /// is of no use for outside users.
    pub fn into_http_response_string(self) -> String {
        format!(
            "HTTP/1.1 {}\n{}\ncontent-length: {}\n\n{}",
            format!(
                "{} {:?}",
                <HttpStatusCode as Into<usize>>::into(self.status_code),
                self.status_code
            ),
            self.headers_to_string(),
            self.body.len(),
            self.body
        )
    }

    fn headers_to_string(&self) -> String {
        self.headers.iter()
            .map(|(hn, value)| {
                format!("{}: {}", <HttpHeaderName as Into<&str>>::into(*hn), value)
            }).collect::<Vec<String>>()
            .join("\n")
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Self {
        let mut r = Response::default();
        r.set_html(s);
        r
    }
}
