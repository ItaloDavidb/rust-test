mod controller;
mod service;
mod utils;
use controller::*;
use service::*;
use std::net::TcpListener;
#[macro_use]
extern crate serde_derive;

fn main() {
    if let Err(e) = set_database() {
        println!("Error setting up database: {}", e);
        return;
    }
    let listener = TcpListener::bind(format!("0.0.0.0:3000")).unwrap();
    println!("Server started at port 3000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established");
                handle_client(stream);
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}