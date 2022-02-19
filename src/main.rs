#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate base64;

use rocket::Config;

mod catchers;
mod fairings;
mod guards;
mod routes;

use fairings::powered_by;
use routes::{base64 as base_64, binary, token};

#[get("/")]
fn home() -> &'static str {
    "Hello User!"
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
        .attach(powered_by::PoweredBy::default())
        // Register the Error catchers
        .register(
            "/",
            catchers![
                catchers::not_found,
                catchers::backend_flipped,
                catchers::you_shall_not_pass,
                catchers::nice_try,
                catchers::broken_request
            ],
        )
        // Mount routes
        .mount("/base64/", routes![base_64::encode64, base_64::decode64])
        .mount(
            "/binary/",
            routes![binary::decode_binary, binary::encode_binary],
        )
        .mount("/", routes![home, token::home, token::header_test])
}
