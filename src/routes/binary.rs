use rocket::{http::Status, response::status};
use std::num::ParseIntError;

fn binary_decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    return (0..s.len())
        .step_by(9)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
        .collect();
}

#[get("/encode?<text>")]
pub fn encode_binary(text: &str) -> status::Custom<String> {
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

#[get("/decode?<data>")]
pub fn decode_binary(data: &str) -> status::Custom<String> {
    if data.to_string().is_empty() {
        return status::Custom(Status::BadRequest, "data parameter is required".to_string());
    }

    let decoded = binary_decode(data).unwrap();

    return status::Custom(Status::Ok, String::from_utf8(decoded).unwrap());
}
