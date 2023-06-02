use std::collections::HashMap;

use chrono::{DateTime, Utc};
use log::error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, routes, uri, Route};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::controllers::taxi_ranks;
use crate::controllers::Error;
use crate::data::JourneyDataProvider;
use crate::guards::User;
use crate::usecase::{
    CancelJourney, CloseJourney, DoesBookingExistsOnJourney, GetAllJourney, GetInProgressJourney,
    GetJourney, HasAJourneyInProgress, PerformJourney,
};
use crate::{entity, Link, BASE_URL};

const TARGET: &'static str = "JOURNEY_CONTROLLER";

#[derive(Deserialize, Validate)]
struct Criteria<'r> {
    departure_schedule: DateTime<Utc>,
    #[validate(url)]
    departure_id: &'r str,
    #[validate(url)]
    arrival_id: &'r str,
}

#[derive(Serialize)]
pub struct Trips {
    trips: Vec<Journey>,
}

#[derive(Serialize)]
pub struct Journey {
    pub id: Uuid,
    pub departure_id: String,
    pub arrival_id: String,
    pub reserved_seats: i32,
    pub departure_schedule: DateTime<Utc>,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

struct JourneyLinksOpts {
    is_cancellable: bool,
    is_closable: bool,
}

impl From<&entity::Journey> for JourneyLinksOpts {
    fn from(value: &entity::Journey) -> Self {
        JourneyLinksOpts {
            is_closable: !value.closed,
            is_cancellable: !value.closed && value.reserved_seats == 0,
        }
    }
}

impl Journey {
    fn links(&mut self, taxi: &str, opts: JourneyLinksOpts) {
        self.links.insert(
            "self",
            Link {
                href: uri!(BASE_URL, show(taxi, &self.id)).to_string(),
            },
        );

        if opts.is_cancellable {
            self.links.insert(
                "cancel",
                Link {
                    href: uri!(BASE_URL, cancel(taxi, &self.id)).to_string(),
                },
            );
        }

        if opts.is_closable {
            self.links.insert(
                "close",
                Link {
                    href: uri!(BASE_URL, close(taxi, &self.id)).to_string(),
                },
            );
        }
    }
}

impl From<entity::Journey> for Journey {
    fn from(value: entity::Journey) -> Self {
        Journey {
            id: value.id,
            reserved_seats: value.reserved_seats,
            departure_id: uri!(BASE_URL, taxi_ranks::show(value.origin)).to_string(),
            arrival_id: uri!(BASE_URL, taxi_ranks::show(value.destination)).to_string(),
            departure_schedule: value.departure_schedule,
            links: HashMap::new(),
        }
    }
}

impl Into<entity::JourneyCriteria> for Criteria<'_> {
    fn into(self) -> entity::JourneyCriteria {
        entity::JourneyCriteria {
            origin: get_stand_id_from_url(self.departure_id),
            destination: get_stand_id_from_url(self.arrival_id),
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
    fn new(id: Uuid, taxi: &str) -> Self {
        PerformJourneyResponse {
            id,
            links: HashMap::from([(
                "self",
                Link {
                    href: uri!(BASE_URL, show(taxi, &id)).to_string(),
                },
            )]),
        }
    }
}

#[post("/taxis/<number>/start-journey", data = "<criteria>")]
async fn start(
    number: &str,
    criteria: Json<Criteria<'_>>,
    user: Result<User, Error>,
    mut data_provider: JourneyDataProvider,
) -> Result<status::Created<Json<PerformJourneyResponse>>, Error> {
    user?;

    data_provider
        .has_a_journey_in_progress(number)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .and_then(|b| {
            if b {
                return Err(Error::has_in_progress_journey());
            }
            Ok(())
        })?;

    let jc: entity::JourneyCriteria = criteria.0.into();
    match data_provider.perform_journey(number, &jc).await {
        Err(err) => {
            error!(target: TARGET, "{err:?}");
            Err(Error::server_error())
        }
        Ok(journey_id) => Ok(
            status::Created::new("").body(Json(PerformJourneyResponse::new(journey_id, number)))
        ),
    }
}

#[get("/taxis/<number>/in-progress-journey", rank = 1)]
async fn in_progress(
    number: &str,
    user: Result<User, Error>,
    mut data_provider: JourneyDataProvider,
) -> Result<Json<Journey>, Error> {
    user?;

    data_provider
        .get_in_progress_journey(number)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .and_then(|j| match j {
            None => Err(Error::no_in_progress_journey()),
            Some(jn) => {
                let opts = JourneyLinksOpts::from(&jn);
                let mut journey = Journey::from(jn);
                journey.links(number, opts);

                Ok(Json(journey))
            }
        })
}

#[get("/taxis/<number>/journey/<id>", rank = 3)]
async fn show(
    number: &str,
    id: Uuid,
    mut data_provider: JourneyDataProvider,
) -> Result<Json<Journey>, Error> {
    data_provider
        .get_journey(&id)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .and_then(|res| match res {
            None => Err(Error::unknown_journey(&id)),
            Some(j) => Ok(j),
        })
        .map(|r| {
            let opts = JourneyLinksOpts::from(&r);
            let mut journey = Journey::from(r);
            journey.links(number, opts);

            Json(journey)
        })
}

#[delete("/taxis/<_number>/journey/<journey>/cancel")]
async fn cancel(
    _number: &str,
    journey: Uuid,
    user: Result<User, Error>,
    mut data_provider: JourneyDataProvider,
) -> Result<status::NoContent, Error> {
    user?;

    data_provider
        .does_booking_exists_on_journey(&journey)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");

            Error::server_error()
        })
        .and_then(|r| {
            if r {
                Err(Error::unable_to_cancel_journey(&journey))
            } else {
                Ok(())
            }
        })?;

    data_provider
        .cancel_journey(&journey)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })?;

    Ok(status::NoContent)
}

#[patch("/taxis/<_number>/journey/<journey_id>/close")]
async fn close(
    _number: &str,
    journey_id: Uuid,
    user: Result<User, Error>,
    mut data_provider: JourneyDataProvider,
) -> Result<Status, Error> {
    user?;

    data_provider
        .close_journey(&journey_id)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })?;

    Ok(Status::Ok)
}

#[get("/taxis/<number>/trips", rank = 2)]
async fn list(
    number: &str,
    user: Result<User, Error>,
    mut data_provider: JourneyDataProvider,
) -> Result<Json<Trips>, Error> {
    user?;

    data_provider
        .get_all_journey(number)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .map(|trips| {
            trips
                .into_iter()
                .map(|j| {
                    let opts = JourneyLinksOpts::from(&j);
                    let mut journey = Journey::from(j);
                    journey.links(number, opts);
                    journey
                })
                .collect()
        })
        .map(|trips| Json(Trips { trips }))
}

pub fn routes() -> Vec<Route> {
    routes![start, cancel, close, show, list, in_progress]
}

fn get_stand_id_from_url(url: &str) -> Uuid {
    url.split("/").last().unwrap().parse().unwrap()
}
