use rocket::http::uri::Absolute;
use rocket::uri;
use serde::Serialize;

pub use controllers::routes;

mod controllers;
mod data;
mod entity;
mod errors;
mod password;
mod usecase;
mod guards;

const BASE_URL: Absolute<'static> = uri!("http://localhost:8000");

#[derive(Serialize)]
struct Link {
    href: String,
}
