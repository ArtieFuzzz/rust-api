use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("{} Is not a valid Route.", req.uri())
}

#[catch(500)]
pub fn backend_flipped(_req: &Request) -> String {
    format!("{}", "Something went wrong in the Backend!")
}

#[catch(401)]
pub fn nice_try(_req: &Request) -> String {
    format!("{}", "Nice try, you aren't authorized to be here.")
}

#[catch(403)]
pub fn you_shall_not_pass(_req: &Request) -> String {
    format!("{}", "Thou shall not pass without authorization.")
}

#[catch(400)]
pub fn broken_request(_req: &Request) -> String {
    format!(
        "{}",
        "There's something wrong with your request. Are you missing a parameter / query or header?"
    )
}
