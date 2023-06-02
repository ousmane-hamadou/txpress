use rocket::Route;

pub use errors::Error;

mod errors;
mod journey;
mod login;
mod registration;
mod taxi_ranks;
mod user;

pub fn routes() -> Vec<Route> {
    let mut routes = vec![];
    routes.extend(registration::routes());
    routes.extend(user::routes());
    routes.extend(login::routes());
    routes.extend(journey::routes());
    routes.extend(taxi_ranks::routes());

    routes
}
