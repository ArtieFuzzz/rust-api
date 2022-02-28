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
use rocket::response::content::RawHtml;
use rocket::shield::{Referrer, Shield, XssFilter};
use routes::{base64 as base_64, binary, token};

#[get("/")]
fn home() -> RawHtml<&'static str> {
    RawHtml(
        "<h1>Hewwo! Welcome to the backend API</h1>
    <p>Right now there is not documentation, why?, Cuz no >:3</p>",
    )
}

#[launch]
fn rocket() -> _ {
    let armour = Shield::default()
        .enable(XssFilter::Disable)
        .enable(Referrer::NoReferrer);

    let config = Config {
        port: 8080,
        keep_alive: 60,
        ..Config::debug_default()
    };

    rocket::build()
        .configure(config)
        .attach(powered_by::PoweredBy::default())
        .attach(armour)
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
