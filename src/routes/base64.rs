use base64::{decode as d64, encode as e64};
use rocket::{http::Status, response::status};
use std::str;

#[get("/encode?<text>")]
pub fn encode64(text: &str) -> status::Custom<String> {
    let string = text.to_string();

    if string.is_empty() {
        return status::Custom(Status::BadRequest, String::from("text parameter required"));
    }

    return status::Custom(Status::Ok, e64(text.to_string()).to_string());
}

#[get("/decode?<data>")]
pub fn decode64(data: &str) -> status::Custom<String> {
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
