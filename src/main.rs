use std::io;
use crate::server::Server;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod server;

fn main() -> io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    let server = Server::new(host, port);
    server.start()?;

    println!("Server started in {}:{}", host, port);

    Ok(())
}

