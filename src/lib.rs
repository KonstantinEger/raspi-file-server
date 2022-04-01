mod raspi_file_server;

use std::error::Error;
use std::net::TcpStream;

use raspi_file_server::{request::Request, response::Response};


pub fn handle_request(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let _request = Request::try_from(&stream)?;
    let response = Response::from(stream)
        .html("<h1>Hello World</h1>");

    response.send()?;
    Ok(())
}
