#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate base64;

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::{Config, Request};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::str;

mod catchers;
mod fairings;
mod routes;

use fairings::powered_by;
use routes::{base64 as base_64, binary};

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

#[get("/")]
fn home() -> &'static str {
    "Hello User!"
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let header = request.headers().get_one("Authorization").unwrap_or("");

        if header.is_empty() {
            return Outcome::Failure((Status::BadRequest, ApiTokenError::Missing));
        }

        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();

        let claims: BTreeMap<String, String> =
            header.verify_with_key(&key).unwrap_or(BTreeMap::new());

        if claims.is_empty() {
            return Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid));
        }

        return Outcome::Success(Token(header.to_string()));
    }
}

#[get("/token")]
fn token() -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");
    let token_string = claims.sign_with_key(&key).unwrap();

    return token_string;
}

#[get("/token/header")]
fn token_header(_token: Token) -> String {
    String::from("Hello!")
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
        .mount("/", routes![home, token, token_header])
}
