#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate base64;

use base64::{decode as d64, encode as e64};
use rocket::http::Status;
use rocket::response::status;
use rocket::{Config, Request};
use std::str;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("{} Is not a valid Route", req.uri())
}

#[catch(500)]
fn backend_flipped(_req: &Request) -> String {
    format!("{}", "Something went wrong in the Backend!")
}

#[get("/")]
fn home() -> &'static str {
    "Hello User!"
}

#[get("/encode?<text>")]
fn encode(text: &str) -> status::Custom<String> {
    let string = text.to_string();

    if string.is_empty() {
        return status::Custom(Status::BadRequest, String::from("text parameter required"));
    }

    return status::Custom(Status::Ok, e64(text.to_string()).to_string());
}

#[get("/decode?<data>")]
fn decode(data: &str) -> status::Custom<String> {
    if data.to_string().is_empty() {
        return status::Custom(Status::BadRequest, "data parameter required".to_string());
    }

    let bytes = &d64(data.to_string()).unwrap_or(Vec::new());

    if bytes.len() == 0 {
        return status::Custom(Status::BadRequest, "invalid base64 data".to_string());
    }

    let decoded = str::from_utf8(bytes);

    return status::Custom(Status::Ok, decoded.unwrap().to_string());
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: 8080,
        keep_alive: 60,
        ..Config::debug_default()
    };

    rocket::build()
        .configure(config)
        .register("/", catchers![not_found, backend_flipped])
        .mount("/", routes![encode, decode, home])
}
