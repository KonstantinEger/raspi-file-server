# raspi-file-server
A file server with web interface, written in Rust.

## Documentation

The TCP Server library is documented via doc comments. To view the docs install
Rust via [rust-up](https://www.rust-lang.org/tools/install), which ships
with cargo. Then run

```bash
$ cargo doc --open
```

## API of the TCP server

```rust
use raspi_file_server::{Server, HttpMethod, Request, Response};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Server::new()
        .add_route(HttpMethod::GET, "/greet/{name}", greet)
        .add_route(HttpMethod::GET, "/", index)
        .bind_and_run("127.0.0.1:8080")?;
}

fn greet(req: Request) -> Response {
    req.params()
        .get("name")
        .map(|n| format!("Hello {}!", name))
        .into()
}

fn index(_: Request) -> Response {
    "Lorem Ipsum".into()
}
```

## Contributing
Feel free to contact me if you'd like to contribute. 
