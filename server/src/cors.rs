use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::options;
use rocket::{Request, Response};
use std::path::PathBuf;

#[options("/<_p..>")]
pub fn for_cors(_p: PathBuf) -> Result<(Status, &'static str), ()> {
    Ok((Status::Ok, ""))
}
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:3000",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Cookie",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
