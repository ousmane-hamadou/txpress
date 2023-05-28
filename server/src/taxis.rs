use std::collections::HashMap;

use rocket::http::{Cookie, CookieJar, Header};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{get, post, routes, uri, Responder, Route};
use serde::{Deserialize, Serialize};
use validator::Validate;

use data_providers::Taxis;
use domain::entity;
use domain::entity::DepartureSchedule;
use domain::usecase::taxi::{AddTaxi, GetTaxi, GetTaxiOwner, PerformJourney};

use crate::errors::Error;
use crate::utils::Link;
use crate::{guards, password, BASE_URL};

#[derive(Deserialize)]
struct Criteria {
    departure_id: Uuid,
    arrival_id: Uuid,
    departure_schedule: DepartureSchedule,
}

impl Into<entity::JourneyCriteria> for Criteria {
    fn into(self) -> entity::JourneyCriteria {
        entity::JourneyCriteria {
            origin: self.departure_id,
            destination: self.arrival_id,
            departure_schedule: self.departure_schedule,
        }
    }
}

#[derive(Serialize)]
struct PerformJourneyResponse {
    id: Uuid,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl PerformJourneyResponse {
    fn new(id: Uuid, taxi_number: &str) -> Self {
        PerformJourneyResponse {
            id,
            links: HashMap::from([
                (
                    "self",
                    Link {
                        href: uri!(BASE_URL, show_journey(taxi_number, &id)).to_string(),
                    },
                ),
                (
                    "bookings",
                    Link {
                        href: format!("{BASE_URL}/bookings?journey_id={id}"),
                    },
                ),
            ]),
        }
    }
}

#[post("/taxis/<number>/perform-journey", data = "<criteria>")]
async fn perform_journey(
    number: &str,
    criteria: Json<Criteria>,
    mut taxis: Taxis,
    owner: Result<guards::TaxiOwner, Error>,
) -> Result<status::Created<Json<PerformJourneyResponse>>, Error> {
    owner?;
    let jc: entity::JourneyCriteria = criteria.0.into();

    match taxis.perform_journey(number, &jc).await {
        None => Err(Error::ServerError),
        Some(journey_id) => Ok(status::Created::new(
            uri!(BASE_URL, show_journey(number, journey_id)).to_string(),
        )
        .body(Json(PerformJourneyResponse::new(journey_id, number)))),
    }
}

#[get("/taxis/<number>/trips/<journey_id>")]
async fn show_journey(
    number: &str,
    journey_id: Uuid,
    owner: Result<guards::TaxiOwner, Error>,
) -> Result<(), Error> {
    owner?;

    println!("{number} - {journey_id}");
    todo!()
}

#[derive(Deserialize, Validate)]
struct TaxiOwner<'r> {
    full_name: &'r str,
    #[validate(length(min = 8, max = 125))]
    password: &'r str,
}

impl Into<entity::TaxiOwner> for &Taxi<'_> {
    fn into(self) -> entity::TaxiOwner {
        entity::TaxiOwner {
            password: self.owner.password.into(),
            full_name: self.owner.full_name.into(),
        }
    }
}

#[derive(Deserialize, Validate)]
struct Taxi<'r> {
    #[validate(length(min = 8, max = 8))]
    number: &'r str,
    brand: &'r str,
    number_of_seats: u8,
    owner: TaxiOwner<'r>,
}

impl Into<entity::Taxi> for &Taxi<'_> {
    fn into(self) -> entity::Taxi {
        entity::Taxi {
            number: self.number.into(),
            number_of_seats: self.number_of_seats as i32,
            brand: self.brand.into(),
        }
    }
}

#[derive(Serialize)]
struct Registration {
    id: String,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Registration {
    fn new(id: &str) -> Self {
        Registration {
            id: id.into(),
            links: HashMap::from([
                (
                    "owner",
                    Link {
                        href: uri!(BASE_URL, taxi_owner(id)).to_string(),
                    },
                ),
                (
                    "self",
                    Link {
                        href: uri!(BASE_URL, show_taxi(id)).to_string(),
                    },
                ),
            ]),
        }
    }
}

#[derive(Responder)]
struct RegistrationResponse {
    inner: Json<Registration>,
    header: Header<'static>,
}

impl RegistrationResponse {
    fn new(reg: Registration) -> Self {
        RegistrationResponse {
            inner: Json(reg),
            header: Header::new("Access-Control-Expose-Headers", "Set-Cookie"),
        }
    }
}

#[post("/taxis/registration", data = "<taxi>")]
async fn registration(
    mut taxi: Json<Taxi<'_>>,
    mut taxis: Taxis,
    cookies: &CookieJar<'_>,
) -> Result<status::Created<RegistrationResponse>, Error> {
    taxi.validate()?;
    taxi.owner.validate()?;

    let hash_password = password::hash(taxi.owner.password.into()).await?;
    taxi.owner.password = hash_password.as_str();

    let owner: entity::TaxiOwner = (&taxi.0).into();
    let taxi: entity::Taxi = (&taxi.0).into();

    if !taxis.add_taxi(&taxi, &owner).await {
        return Err(Error::ServerError);
    }

    let payload = serde_json::json!({"full_name": owner.full_name});
    cookies.add_private(Cookie::new(taxi.number.clone(), payload.to_string()));

    let res = RegistrationResponse::new(Registration::new(&taxi.number));
    Ok(status::Created::new(uri!(BASE_URL, show_taxi(&taxi.number)).to_string()).body(res))
}

#[derive(Deserialize)]
struct Credentials<'r> {
    id: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
struct User {
    name: String,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl User {
    fn new(name: &str, taxi_num: &str) -> Self {
        User {
            name: name.into(),
            links: HashMap::from([
                (
                    "self",
                    Link {
                        href: uri!(BASE_URL, taxi_owner(taxi_num)).to_string(),
                    },
                ),
                (
                    "taxis",
                    Link {
                        href: uri!(BASE_URL, show_taxi(taxi_num)).to_string(),
                    },
                ),
                (
                    "perform_journey",
                    Link {
                        href: uri!(BASE_URL, perform_journey(taxi_num)).to_string(),
                    },
                ),
            ]),
        }
    }
}

#[post("/taxis/login", data = "<credentials>")]
async fn login(
    credentials: Json<Credentials<'_>>,
    mut taxis: Taxis,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>, Error> {
    match taxis.get_taxi_owner(credentials.id).await {
        None => Err(Error::InvalidCredentials(credentials.id.into())),
        Some(user) => {
            if password::verify(credentials.password, &user.password).await? {
                let payload = serde_json::json!({"full_name": user.full_name});
                cookies.add_private(Cookie::new(credentials.id.to_owned(), payload.to_string()));
                return Ok(Json(User::new(&user.full_name, credentials.id)));
            }

            Err(Error::InvalidCredentials(credentials.id.into()))
        }
    }
}

#[derive(Serialize)]
struct Owner {
    name: String,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Into<Owner> for entity::TaxiOwner {
    fn into(self) -> Owner {
        Owner {
            name: self.full_name,
            links: HashMap::new(),
        }
    }
}

#[get("/taxis/<number>/owner")]
async fn taxi_owner(number: &str, mut taxis: Taxis) -> Result<Json<Owner>, Error> {
    match taxis.get_taxi_owner(number).await {
        None => Err(Error::UnknownTaxi(number.into())),
        Some(o) => {
            let mut owner: Owner = o.into();
            owner.links.insert(
                "self",
                Link {
                    href: uri!(BASE_URL, taxi_owner(number)).to_string(),
                },
            );
            owner.links.insert(
                "taxi",
                Link {
                    href: uri!(BASE_URL, show_taxi(number)).to_string(),
                },
            );
            owner.links.insert(
                "perform_journey",
                Link {
                    href: uri!(BASE_URL, perform_journey(number)).to_string(),
                },
            );

            Ok(Json(owner))
        }
    }
}

#[derive(Serialize)]
struct ShowTaxi {
    number: String,
    number_of_seats: u8,
    brand: String,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Into<ShowTaxi> for entity::Taxi {
    fn into(self) -> ShowTaxi {
        ShowTaxi {
            brand: self.brand,
            number_of_seats: self.number_of_seats as u8,
            links: HashMap::from([
                (
                    "self",
                    Link {
                        href: uri!(BASE_URL, show_taxi(&self.number)).to_string(),
                    },
                ),
                (
                    "owner",
                    Link {
                        href: uri!(BASE_URL, taxi_owner(&self.number)).to_string(),
                    },
                ),
            ]),
            number: self.number,
        }
    }
}

#[get("/taxis/<number>")]
async fn show_taxi(number: &str, mut taxis: Taxis) -> Result<Json<ShowTaxi>, Error> {
    match taxis.get_taxi(number).await {
        None => Err(Error::UnknownTaxi(number.into())),
        Some(taxi) => Ok(Json(taxi.into())),
    }
}

pub fn routes() -> Vec<Route> {
    routes![
        show_journey,
        perform_journey,
        registration,
        taxi_owner,
        show_taxi,
        login
    ]
}
