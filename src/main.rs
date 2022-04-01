use std::{net, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = net::TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        raspi_file_server::handle_request(stream?)?;
    }

    Ok(())
}
