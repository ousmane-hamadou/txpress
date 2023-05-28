use crate::cors::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN};
use rocket::http::uri::Absolute;
use rocket::http::{Header, Method};
use rocket::{uri, Build, Rocket};

mod cors;
mod errors;
mod guards;
mod password;
mod searches;
mod taxi_ranks;
mod taxis;
mod utils;

const BASE_URL: Absolute<'static> = uri!("http://localhost:8000");

pub fn server() -> Rocket<Build> {
    rocket::build()
        .attach(data_providers::init())
        .attach(rocket::fairing::AdHoc::on_response(
            "SET_CORS",
            |req, res| {
                Box::pin(async {
                    match req.method() {
                        Method::Post => {
                            res.set_header(Header::new(
                                ACCESS_CONTROL_ALLOW_ORIGIN,
                                "http://localhost:3000",
                            ));
                            res.set_header(Header::new(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"));
                        }
                        _ => (),
                    }
                })
            },
        ))
        .mount("/", cors::cors())
        .mount("/", taxi_ranks::routes()) // stand
        .mount("/", taxis::routes()) // taxis
        .mount("/", searches::routes()) // searches
}
