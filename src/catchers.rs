use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("{} Is not a valid Route", req.uri())
}

#[catch(500)]
pub fn backend_flipped(_req: &Request) -> String {
    format!("{}", "Something went wrong in the Backend!")
}
