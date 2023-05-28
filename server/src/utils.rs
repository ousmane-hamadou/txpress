use rocket::serde::uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct Link {
    pub href: String,
}

pub fn station_id_from_url(url: &str) -> Uuid {
    url.split("/").last().unwrap().parse().unwrap()
}
