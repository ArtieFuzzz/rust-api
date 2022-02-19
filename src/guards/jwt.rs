use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};
use sha2::Sha256;

pub struct Token(String);

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
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
