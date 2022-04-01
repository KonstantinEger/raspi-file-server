use std::net::TcpStream;

pub struct Request;

impl From<&TcpStream> for Request {
    fn from(_: &TcpStream) -> Self {
        Request
    }
}