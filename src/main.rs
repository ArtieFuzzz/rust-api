#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate base64;

use base64::{decode as d64, encode as e64};
use rocket::http::Status;
use rocket::response::status;
use rocket::{Config, Request};
use std::num::ParseIntError;
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
fn encode64(text: &str) -> status::Custom<String> {
    let string = text.to_string();

    if string.is_empty() {
        return status::Custom(Status::BadRequest, String::from("text parameter required"));
    }

    return status::Custom(Status::Ok, e64(text.to_string()).to_string());
}

#[get("/decode?<data>")]
fn decode64(data: &str) -> status::Custom<String> {
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

#[get("/encode?<text>")]
fn encode_binary(text: &str) -> status::Custom<String> {
    if text.to_string().is_empty() {
        return status::Custom(Status::BadRequest, "text parameter is required".to_string());
    }

    let mut binary = "".to_string();
    let string = text.to_string();

    for char in string.clone().into_bytes() {
        binary += &format!("0{:b} ", char)
    }

    return status::Custom(Status::Ok, format!("{}", binary));
}

fn binary_decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    return (0..s.len())
        .step_by(9)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
        .collect();
}

#[get("/decode?<data>")]
fn decode_binary(data: &str) -> status::Custom<String> {
    if data.to_string().is_empty() {
        return status::Custom(Status::BadRequest, "data parameter is required".to_string());
    }

    let decoded = binary_decode(data).unwrap();

    return status::Custom(Status::Ok, String::from_utf8(decoded).unwrap());
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
        .mount("/base64/", routes![encode64, decode64])
        .mount("/binary/", routes![encode_binary, decode_binary])
        .mount("/", routes![home])
}
