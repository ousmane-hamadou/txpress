use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rocket::http::{Cookie, CookieJar, Header, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{get, post, routes, uri, Route};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use validator::Validate;

use data_providers::Searches;
use domain::entity;
use domain::entity::SearchedTaxi;
use domain::usecase::search::{FetchTaxiByJourneyCriteria, GetSelectedTaxi};

use crate::errors::Error;
use crate::taxis;
use crate::utils::{station_id_from_url, Link};
use crate::BASE_URL;

#[derive(Serialize)]
pub struct Selection {
    taxi: Taxi,
    reserved_seats: u8,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl Selection {
    fn new(search_id: &Uuid, selected_taxi: SearchedTaxi, reserved_seats: u8) -> Self {
        let mut taxi = Taxi::new(search_id, selected_taxi);
        taxi.links = serde_json::Value::Null;

        Selection {
            reserved_seats,
            taxi,
            links: HashMap::from([
                (
                    "self",
                    Link {
                        href: uri!(BASE_URL, selection(search_id)).to_string(),
                    },
                ),
                (
                    "search",
                    Link {
                        href: uri!(BASE_URL, show(search_id)).to_string(),
                    },
                ),
                (
                    "book",
                    Link {
                        href: format!("{BASE_URL}/bookings/{search_id}"),
                    },
                ),
            ]),
        }
    }
}

#[derive(Validate, Deserialize)]
struct Seats {
    #[validate(range(min = 1))]
    seats: u8,
}

#[derive(Serialize)]
struct TaxiList {
    taxis: Vec<Taxi>,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

impl TaxiList {
    fn new(search_id: &Uuid, taxis: Vec<Taxi>) -> Self {
        TaxiList {
            taxis,
            links: HashMap::from([
                (
                    "self",
                    Link {
                        href: uri!(BASE_URL, list_taxis(search_id)).to_string(),
                    },
                ),
                (
                    "search",
                    Link {
                        href: uri!(BASE_URL, show(search_id)).to_string(),
                    },
                ),
            ]),
        }
    }
}

#[derive(Serialize)]
struct Taxi {
    number: String,
    number_of_seats: u8,
    brand: String,
    departure_schedule: DateTime<Utc>,
    available_seats: u8,
    #[serde(rename = "_links", skip_serializing_if = "serde_json::Value::is_null")]
    links: serde_json::Value,
}

impl Taxi {
    fn new(search_id: &Uuid, searched_taxi: SearchedTaxi) -> Self {
        let available_seats = (searched_taxi.number_of_seats - searched_taxi.available_seats) as u8;
        let mut link = HashMap::new();
        for i in 1..=available_seats {
            link.insert(
                i,
                Link {
                    href: uri!(
                        BASE_URL,
                        select_taxi(
                            search_id,
                            &searched_taxi.number,
                            i,
                            &searched_taxi.journey_id
                        )
                    )
                    .to_string(),
                },
            );
        }
        Taxi {
            brand: searched_taxi.brand,
            number_of_seats: searched_taxi.number_of_seats as u8,
            departure_schedule: searched_taxi.departure_schedule,
            available_seats,
            links: json!({
                "self": {
                    "href": uri!(BASE_URL, taxis::show_taxi(&searched_taxi.number)).to_string()
                },
                "select": link
            }),
            number: searched_taxi.number,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Selected {
    taxi: String,
    journey_id: Uuid,
    seats: u8,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct Search {
    id: Uuid,
    criteria: SearchCriteria,
    selection: Option<Selected>,
}

impl Search {
    fn new(id: Uuid, criteria: SearchCriteria, selection: Option<Selected>) -> Self {
        Search {
            id,
            criteria,
            selection,
        }
    }

    fn from(value: Cookie<'_>) -> Self {
        serde_json::from_str::<Search>(value.value()).unwrap()
    }
}

impl Into<SearchResource> for Search {
    fn into(self) -> SearchResource {
        let mut links = HashMap::from([
            (
                "self",
                Link {
                    href: uri!(BASE_URL, show(&self.id)).to_string(),
                },
            ),
            (
                "taxis",
                Link {
                    href: uri!(BASE_URL, list_taxis(&self.id)).to_string(),
                },
            ),
        ]);
        if let Some(_) = self.selection {
            links.insert(
                "selection",
                Link {
                    href: uri!(BASE_URL, selection(&self.id)).to_string(),
                },
            );
        }

        SearchResource {
            id: self.id,
            criteria: self.criteria.into(),
            links,
        }
    }
}

#[derive(Serialize)]
struct SearchResource {
    id: Uuid,
    criteria: SearchCriteria,
    #[serde(rename = "_links")]
    links: HashMap<&'static str, Link>,
}

#[derive(Responder)]
struct PerformSearchResponse<'r> {
    inner: Json<SearchResource>,
    header: Header<'r>,
}

impl PerformSearchResponse<'_> {
    fn new(inner: SearchResource) -> Self {
        PerformSearchResponse {
            inner: Json(inner),
            header: Header::new("Access-Control-Expose-Headers", "Set-Cookie"),
        }
    }
}

impl<'r> Into<Cookie<'r>> for &Search {
    fn into(self) -> Cookie<'r> {
        let payload = serde_json::to_string(self).unwrap();

        Cookie::build(self.id.to_string(), payload)
            .http_only(true)
            .path("/")
            .finish()
    }
}

#[derive(Deserialize, Serialize, Validate)]
struct SearchCriteria {
    #[validate(url)]
    departure_id: String,
    #[validate(url)]
    arrival_id: String,
}

impl Into<entity::SearchCriteria> for &SearchCriteria {
    fn into(self) -> entity::SearchCriteria {
        entity::SearchCriteria {
            arrival: station_id_from_url(&self.arrival_id),
            departure: station_id_from_url(&self.departure_id),
        }
    }
}

#[post("/searches", data = "<criteria>")]
async fn perform_search(
    criteria: Json<SearchCriteria>,
    cookies: &CookieJar<'_>,
) -> Result<(Status, PerformSearchResponse<'static>), Error> {
    criteria.validate()?;

    let s = Search::new(Uuid::new_v4(), criteria.0, None);

    cookies.add_private((&s).into());
    Ok((Status::Created, PerformSearchResponse::new(s.into())))
}

#[get("/searches/<id>")]
async fn show(id: Uuid, cookies: &CookieJar<'_>) -> Result<Json<SearchResource>, Error> {
    cookies
        .get_private(&id.to_string())
        .map(Search::from)
        .map(|s| Json(s.into()))
        .ok_or_else(move || Error::UnknownSearch(id))
}

#[get("/searches/<id>/taxis")]
async fn list_taxis(
    id: Uuid,
    cookies: &CookieJar<'_>,
    mut searches: Searches,
) -> Result<Json<TaxiList>, Error> {
    let search = cookies
        .get_private(&id.to_string())
        .map(Search::from)
        .ok_or_else(move || Error::UnknownSearch(id))?;

    let taxis: Vec<_> = searches
        .fetch_taxis(&(&search.criteria).into())
        .await
        .into_iter()
        .map(|t| Taxi::new(&id, t))
        .collect();

    Ok(Json(TaxiList::new(&id, taxis)))
}

#[get("/searches/<id>/selection")]
async fn selection(
    id: Uuid,
    cookies: &CookieJar<'_>,
    mut searches: Searches,
) -> Result<Json<Selection>, Error> {
    let search = cookies
        .get_private(&id.to_string())
        .map(Search::from)
        .ok_or_else(|| Error::UnknownSearch(id))?;

    if let None = search.selection {
        return Err(Error::NoSelection(id));
    }

    let ss = searches
        .get_selected_taxi(&search.selection.as_ref().unwrap().journey_id)
        .await;

    Ok(Json(Selection::new(
        &id,
        ss,
        search.selection.as_ref().unwrap().seats,
    )))
}

#[post("/searches/<id>/taxi/<taxi_id>/seats/<seats>/select?<journey>")]
async fn select_taxi(
    id: Uuid,
    taxi_id: String,
    seats: u8,
    cookies: &CookieJar<'_>,
    journey: Uuid,
) -> Result<(Status, PerformSearchResponse<'static>), Error> {
    let mut search = cookies
        .get_private(&id.to_string())
        .map(Search::from)
        .ok_or_else(|| Error::UnknownSearch(id))?;

    search.selection = Some(Selected {
        taxi: taxi_id,
        journey_id: journey,
        seats,
    });

    cookies.add_private((&search).into());
    Ok((Status::Created, PerformSearchResponse::new(search.into())))
}

pub fn routes() -> Vec<Route> {
    routes![perform_search, list_taxis, show, select_taxi, selection]
}
