
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;
use std::thread;
#[get("/")]
fn hello() -> &'static str {
    println!("thread:{:?}", thread::current().id());
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}