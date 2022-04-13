use raspi_file_server::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Server::new()
        .add_route(HttpMethod::GET, "/", index)
        .bind_and_run("127.0.0.1:8080")?;
    Ok(())
}

fn index(_: &Request) -> Response {
    let mut response = Response::default();
    response.set_json("{\"msg\":\"hello world\"}");
    response
}
