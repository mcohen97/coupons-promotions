use std::io;
use crate::server::Server;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate mime;
extern crate chrono;
extern crate iso3166_1;
extern crate iata_types;

mod server;
mod services;
mod models;


fn main() -> io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    let server = Server::new(host, port);
    server.start()?;

    println!("Server started in {}:{}", host, port);

    Ok(())
}

