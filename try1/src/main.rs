#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate ws;
extern crate rand;
#[macro_use]
extern crate serde_json;

use std::thread;
use std::rc::Rc;
use std::cell::RefCell;

mod server;
mod connection;
mod interpreter;

use server::Server;
use connection::ConnectionHandler;
use interpreter::Interpreter;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api")]
fn api() -> &'static str {
    "Hello, api!"
}

fn main() {

    thread::spawn(|| {
        let interp = Rc::new(RefCell::new(Interpreter::new(Server::new())));
        ws::listen("127.0.0.1:5001", |out| ConnectionHandler::new(out, interp.clone()) ).unwrap()
    });

    rocket::ignite()
        .mount("/", routes![index, api])

        .launch();
}