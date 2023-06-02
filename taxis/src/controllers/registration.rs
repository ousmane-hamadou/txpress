use std::collections::HashMap;

use log::error;
use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{post, routes, uri, Route};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::controllers::user;
use crate::controllers::Error;
use crate::data::RegistrationDataProvider;
use crate::entity::{Owner, Taxi};
use crate::usecase::{AddTaxi, DoesTaxiExists};
use crate::{password, Link, BASE_URL};

const TARGET: &'static str = "REGISTRATION_CONTROLLER";

#[derive(Serialize, Deserialize)]
struct PartialTaxiData {
    number: String,
}

#[derive(Deserialize)]
pub struct AdditionalTaxiData {
    owner: String,
    password: String,
    car_brand: String,
    num_of_seats: u8,
}

impl PartialTaxiData {
    fn new(number: String) -> Self {
        PartialTaxiData { number }
    }
}

#[derive(Serialize)]
pub struct StartResponse {
    id: Uuid,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

#[derive(Serialize)]
pub struct FinishResponse {
    id: Uuid,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl FinishResponse {
    fn new(id: Uuid, num: String) -> Self {
        FinishResponse {
            id,
            links: HashMap::from([(
                "self",
                Link {
                    href: uri!(BASE_URL, user::show(num)).to_string(),
                },
            )]),
        }
    }
}

impl StartResponse {
    fn new(id: Uuid) -> Self {
        StartResponse {
            id,
            links: HashMap::from([(
                "next",
                Link {
                    href: uri!(BASE_URL, finish(&id)).to_string(),
                },
            )]),
        }
    }
}

#[post("/taxis/registration?<number>")]
pub async fn start(
    number: &str,
    cookies: &CookieJar<'_>,
    mut data_provider: RegistrationDataProvider,
) -> Result<Json<StartResponse>, Error> {
    data_provider
        .does_taxi_exists(number)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .and_then(|exists| match exists {
            true => Err(Error::taxi_exists(format!(
                "The taxi with number `{number}` exits"
            ))),
            _ => {
                let data = PartialTaxiData::new(number.into());
                let id = Uuid::new_v4();

                cookies.add_private(Cookie::new(
                    id.to_string(),
                    serde_json::to_string(&data).unwrap(),
                ));
                Ok(Json(StartResponse::new(id)))
            }
        })
}

#[post("/taxis/registration/<id>/complete", data = "<data>")]
pub async fn finish(
    id: Uuid,
    data: Json<AdditionalTaxiData>,
    mut data_provider: RegistrationDataProvider,
    cookies: &CookieJar<'_>,
) -> Result<status::Created<Json<FinishResponse>>, Error> {
    let result = cookies
        .get_private(&id.to_string())
        .map(|c| {
            let res = serde_json::from_str::<PartialTaxiData>(c.value()).unwrap();
            cookies.remove_private(c);
            res
        })
        .and_then(|pd| {
            let taxi = Taxi::new(&pd.number, &data.car_brand, data.num_of_seats as i32);

            let owner = Owner::new(&data.owner, &data.password);

            Some((taxi, owner))
        });

    if let None = result {
        return Err(Error::unknown_registration_id(&id));
    }

    match result {
        None => Err(Error::unknown_registration_id(&id)),
        Some((taxi, mut owner)) => {
            let password = password::hash(owner.password).await.unwrap();
            owner.password = password;

            match data_provider.add_taxi(&taxi, &owner).await {
                Err(err) => {
                    error!(target: TARGET, "{err:?}");
                    Err(Error::server_error())
                }

                Ok(_) => {
                    let payload = serde_json::json!({"full_name": &owner.full_name});
                    let s = taxi.number.to_lowercase();

                    cookies.add_private(Cookie::new(s.to_owned(), payload.to_string()));
                    Ok(status::Created::new("").body(Json(FinishResponse::new(taxi.id, s))))
                }
            }
        }
    }
}

pub fn routes() -> Vec<Route> {
    routes![start, finish]
}
