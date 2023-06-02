use std::collections::HashMap;

use log::error;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::{post, routes, uri, Route};
use serde::{Deserialize, Serialize};

use crate::controllers::Error;
use crate::controllers::{journey, user};
use crate::data::LoginDataProvider;
use crate::usecase::GetOwner;
use crate::{password, Link, BASE_URL};

#[derive(Deserialize)]
pub struct Credential<'r> {
    number: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
pub struct Response {
    name: String,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Response {
    fn new(taxi_num: &str, name: &str) -> Self {
        let links = HashMap::from([
            (
                "start-journey",
                Link {
                    href: uri!(BASE_URL, journey::start(taxi_num)).to_string(),
                },
            ),
            (
                "trips",
                Link {
                    href: uri!(BASE_URL, journey::list(taxi_num)).to_string(),
                },
            ),
            (
                "self",
                Link {
                    href: uri!(BASE_URL, user::show(taxi_num)).to_string(),
                },
            ),
            (
                "journey_in_progress",
                Link {
                    href: uri!(BASE_URL, journey::in_progress(taxi_num)).to_string(),
                },
            ),
        ]);
        Response {
            links,
            name: name.into(),
        }
    }
}

#[post("/taxis/login", data = "<credential>")]
async fn index(
    credential: Json<Credential<'_>>,
    mut db: LoginDataProvider,
    cookies: &CookieJar<'_>,
) -> Result<Json<Response>, Error> {
    match db.get_owner(credential.number).await {
        Err(err) => {
            error!(target: "LOGIN_CONTROLLER","{err:?}");
            Err(Error::server_error())
        }
        Ok(op) => match op {
            None => Err(Error::invalid_number(credential.number)),
            Some(u) => match password::verify(credential.password, &u.password)
                .await
                .unwrap()
            {
                false => Err(Error::invalid_password()),
                true => {
                    let payload = serde_json::json!({"full_name": &u.full_name});
                    let num = credential.number.to_lowercase();

                    cookies.add_private(Cookie::new(num.to_owned(), payload.to_string()));

                    Ok(Json(Response::new(&num, &u.full_name)))
                }
            },
        },
    }
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
