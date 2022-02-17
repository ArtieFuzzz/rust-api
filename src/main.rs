#![feature(proc_macro_hygiene, decl_macro)]

// use uri_pct;

#[macro_use]
extern crate rocket;

#[get("/encode?<text>")]
fn encode(text: &str) -> String {
    text.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![encode])
}
