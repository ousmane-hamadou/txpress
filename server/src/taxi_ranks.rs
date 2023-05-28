use std::collections::HashMap;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{get, post, routes, uri, Route};
use serde::{Deserialize, Serialize};

use data_providers::TaxiRanks;
use domain::entity;
use domain::usecase::stand::{AddStand, GetAllStand, GetStand};

use crate::utils::Link;
use crate::BASE_URL;

#[derive(Serialize)]
struct StandList {
    taxi_ranks: Vec<Stand>,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Into<StandList> for Vec<entity::Station> {
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

impl Into<StandResponse> for entity::Station {
    fn into(self) -> StandResponse {
        StandResponse {
            links: links_for_stand(&self.id),
            stand: Stand::from(self),
        }
    }
}

#[derive(Deserialize)]
struct NewTaxiRanks {
    stations: Vec<String>,
}

impl From<entity::Station> for Stand {
    fn from(value: entity::Station) -> Self {
        Stand {
            id: uri!(BASE_URL, show(value.id)).to_string(),
            name: value.name,
        }
    }
}

#[post("/taxi-ranks", data = "<data>")]
async fn add_stations(data: Json<NewTaxiRanks>, mut stations: TaxiRanks) -> Json<StandList> {
    let res = stations.add_stand(data.0.stations).await;
    Json(res.into())
}

#[get("/taxi-ranks")]
async fn list_all(mut stations: TaxiRanks) -> Json<StandList> {
    let items: Vec<_> = stations.get_all_stand().await;
    Json(items.into())
}

#[get("/taxi-ranks/<id>")]
async fn show(mut stations: TaxiRanks, id: Uuid) -> Option<Json<StandResponse>> {
    stations.get_stand(&id).await.map(|s| Json(s.into()))
}

pub fn routes() -> Vec<Route> {
    routes![list_all, show, add_stations]
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
