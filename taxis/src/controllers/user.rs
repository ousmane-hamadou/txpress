use std::collections::HashMap;

use rocket::serde::json::Json;
use rocket::{get, routes, uri, Route};
use serde::Serialize;

use crate::controllers::journey;
use crate::controllers::Error;
use crate::guards::User as UserGuard;
use crate::Link;
use crate::BASE_URL;

#[derive(Serialize)]
pub struct User {
    name: String,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl User {
    fn new(num: &str, name: &str) -> Self {
        let links = HashMap::from([
            (
                "start-journey",
                Link {
                    href: uri!(BASE_URL, journey::start(num)).to_string(),
                },
            ),
            (
                "trips",
                Link {
                    href: uri!(BASE_URL, journey::list(num)).to_string(),
                },
            ),
            (
                "self",
                Link {
                    href: uri!(BASE_URL, show(num)).to_string(),
                },
            ),
            (
                "journey_in_progress",
                Link {
                    href: uri!(BASE_URL, journey::in_progress(num)).to_string(),
                },
            ),
        ]);

        User {
            links,
            name: name.into(),
        }
    }
}

#[get("/taxis/<num>/owner", rank = 4)]
async fn show(num: &str, user: Result<UserGuard, Error>) -> Result<Json<User>, Error> {
    let user = user?;

    Ok(Json(User::new(num, &user.full_name)))
}

pub fn routes() -> Vec<Route> {
    routes![show]
}
