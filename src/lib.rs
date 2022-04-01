mod raspi_file_server;

use std::error::Error;
use std::net::TcpStream;

use raspi_file_server::{request::Request, response::Response};


pub fn handle_request(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let _request = Request::from(&stream);
    let response = Response::from(stream);

    response.send()?;
    Ok(())
}