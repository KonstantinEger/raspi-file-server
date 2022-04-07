mod response;

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::prelude::*;
pub use response::{Response, HttpHeaderName, HttpStatusCode};

#[derive(Default)]
pub struct Server {
    routes: Vec<(HttpMethod, String, Box<dyn Fn(Request) -> Response>)>
}

impl Server {
    /// Creates a new server, which can be configured further and then started
    /// by calling [Server::bind_and_run].
    /// ```
    /// use raspi_file_server::*;
    ///
    /// fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    ///     Server::new()
    ///         .add_route(HttpMethod::GET, "/", |_| Response::default())
    ///         .bind_and_run("127.0.0.1:8080")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an endpoint to the server. A handler takes a request and returns a
    /// [Response]. Response implements [From<&str>], which makes it easy to send
    /// text back to the client.
    /// ```
    /// use raspi_file_server::*;
    ///
    /// fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    ///     Server::new()
    ///         .add_route(HttpMethod::GET, "/", index_route)
    ///         .bind_and_run("127.0.0.1:8080")?;
    ///     Ok(())
    /// }
    ///
    /// fn index_route(_: Request) -> Response {
    ///     "<h1>Index page</h1>".into()
    /// }
    /// ```
    pub fn add_route<F>(&mut self, method: HttpMethod, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Response + 'static
    {
        self.routes.push((method, path.to_string(), Box::new(handler)));
        self
    }

    /// Starts the server, bound to the specified address. The address can be passed
    /// in different formats, which implement [ToSocketAddrs].
    pub fn bind_and_run<A: ToSocketAddrs>(&mut self, address: A) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            self.handle_request(stream?)?;
        }
        Ok(())
    }

    fn handle_request(&self, mut stream: TcpStream) -> std::io::Result<()> {
        // TODO: correct implementation with matching the request to different routes
        // this is just a temporary workaround for testing
        if let Some((_,_,handler)) = self.routes.get(0) {
            let response = (**handler)(Request);
            stream.write(response.into_http_response_string().as_bytes())?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    PUT,
    PATCH,
    DELETE
}

pub struct Request;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_route() {
        let mut server = Server::new();
        assert_eq!(server.routes.len(), 0);
        server.add_route(HttpMethod::GET, "/", |_| Response::default());
        assert_eq!(server.routes.len(), 1);
        let (m, p, _) = server.routes.get(0).unwrap();
        assert_eq!(*m, HttpMethod::GET);
        assert_eq!(*p, "/");
    }
}
