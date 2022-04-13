mod request;
mod response;

pub use request::{HttpMethod, Request};
use response::response_into_http_response_string;
pub use response::{HttpHeaderName, HttpStatusCode, Response};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

type Routes = Vec<(HttpMethod, String, Box<dyn Fn(&Request) -> Response>)>;

#[derive(Default)]
pub struct Server {
    routes: Routes,
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

    /// Adds an endpoint to the server. A handler takes a [Request] and returns a
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
    /// fn index_route(_: &Request) -> Response {
    ///     "<h1>Index page</h1>".into()
    /// }
    /// ```
    pub fn add_route<F>(&mut self, method: HttpMethod, path: &str, handler: F) -> &mut Self
    where
        F: Fn(&Request) -> Response + 'static,
    {
        self.routes
            .push((method, path.to_string(), Box::new(handler)));
        self
    }

    /// Starts the server, bound to the specified address. The address can be passed
    /// in different formats, which implement [ToSocketAddrs].
    pub fn bind_and_run<A: ToSocketAddrs>(&mut self, address: A) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming().filter_map(Result::ok) {
            self.handle_request(stream)?;
        }
        Ok(())
    }

    fn handle_request(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut request = {
            let mut buffer = [0; 5120];
            let _ = stream.read(&mut buffer)?;
            let content = String::from_utf8_lossy(&buffer).to_string();
            let request_result = request::utils::parse_request_from_http_request_body(content);
            if let Err(err) = request_result {
                stream.write_all(response_into_http_response_string(err.into()).as_bytes())?;
                return Ok(());
            }
            request_result.unwrap()
        };

        if let Some((_, route, handler)) = self.routes.iter().find(|(method, route, _)| {
            (*method == request.method()) && request::utils::request_matches_route(&request, route)
        }) {
            request::utils::set_request_params_according_to_match(&mut request, route);
            let response = handler(&request);
            stream.write_all(response_into_http_response_string(response).as_bytes())?;
        }
        Ok(())
    }
}

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
