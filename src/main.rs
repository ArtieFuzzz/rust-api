#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate base64;

use base64::{decode as d64, encode as e64};
use std::str;

#[get("/")]
fn home() -> &'static str {
    "Hello User!"
}

#[get("/encode?<text>")]
fn encode(text: &str) -> String {
    e64(text.to_string()).to_string()
}

#[get("/decode?<data>")]
fn decode(data: &str) -> String {
    str::from_utf8(&d64(data.to_string()).unwrap())
        .unwrap()
        .to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![encode, decode, home])
}
