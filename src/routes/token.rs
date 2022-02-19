use crate::guards::jwt::Token;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

#[get("/token")]
pub fn home() -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");
    let token_string = claims.sign_with_key(&key).unwrap();

    return token_string;
}

#[get("/token/header")]
pub fn header_test(_token: Token) -> String {
    String::from("Hello!")
}
