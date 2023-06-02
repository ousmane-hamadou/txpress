use std::collections::HashMap;

use log::error;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{get, post, routes, uri, Route};
use serde::{Deserialize, Serialize};

use crate::controllers::Error;
use crate::data::StandDataProvider;
use crate::usecase::{AddTaxiRanks, GetAllStand, GetStand};
use crate::{entity, Link, BASE_URL};

const TARGET: &'static str = "TAXI_RANKS_CONTROLLER";

#[derive(Serialize)]
struct StandList {
    taxi_ranks: Vec<Stand>,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Into<StandList> for Vec<entity::Stand> {
    fn into(self) -> StandList {
        StandList {
            taxi_ranks: self.into_iter().map(Stand::from).collect(),
            links: links_for_taxi_ranks(),
        }
    }
}

#[derive(Serialize)]
struct Stand {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct StandResponse {
    stand: Stand,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Into<StandResponse> for entity::Stand {
    fn into(self) -> StandResponse {
        StandResponse {
            links: links_for_stand(&self.id),
            stand: Stand::from(self),
        }
    }
}

#[derive(Deserialize)]
struct NewTaxiRanks {
    taxi_ranks: Vec<String>,
}

impl From<entity::Stand> for Stand {
    fn from(value: entity::Stand) -> Self {
        Stand {
            id: uri!(BASE_URL, show(value.id)).to_string(),
            name: value.name,
        }
    }
}

#[post("/taxi-ranks", data = "<data>")]
async fn add_taxi_ranks(
    data: Json<NewTaxiRanks>,
    mut data_provider: StandDataProvider,
) -> Result<Json<StandList>, Error> {
    data_provider
        .add_taxi_ranks(data.0.taxi_ranks)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .map(|ranks| Json(ranks.into()))
}

#[get("/taxi-ranks")]
async fn list_all(mut data_provider: StandDataProvider) -> Result<Json<StandList>, Error> {
    data_provider
        .get_all_stand()
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .map(|items| Json(items.into()))
}

#[get("/taxi-ranks/<id>")]
async fn show(
    mut data_provider: StandDataProvider,
    id: Uuid,
) -> Result<Json<StandResponse>, Error> {
    data_provider
        .get_stand(&id)
        .await
        .map_err(|err| {
            error!(target: TARGET, "{err:?}");
            Error::server_error()
        })
        .and_then(|s| match s {
            None => Err(Error::unknown_stand(&id)),
            Some(r) => Ok(Json(r.into())),
        })
}

pub fn routes() -> Vec<Route> {
    routes![list_all, show, add_taxi_ranks]
}

fn links_for_taxi_ranks() -> HashMap<&'static str, Link> {
    HashMap::from([(
        "self",
        Link {
            href: uri!(BASE_URL, list_all()).to_string(),
        },
    )])
}

fn links_for_stand(id: &Uuid) -> HashMap<&'static str, Link> {
    HashMap::from([
        (
            "taxi_ranks",
            Link {
                href: uri!(BASE_URL, list_all()).to_string(),
            },
        ),
        (
            "self",
            Link {
                href: uri!(BASE_URL, show(id)).to_string(),
            },
        ),
    ])
}
