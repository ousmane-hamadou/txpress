use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Failure {
    error: &'static str,
    error_description: String,
}

#[derive(Responder, Debug)]
#[response(content_type = "json")]
pub enum Error {
    #[response(status = 400)]
    TaxiExists(Json<Failure>),
    #[response(status = 500)]
    ServerError(Json<Failure>),
    #[response(status = 404)]
    UnknownRegistrationId(Json<Failure>),
    #[response(status = 400)]
    InvalidNumber(Json<Failure>),
    #[response(status = 400)]
    InvalidPassword(Json<Failure>),
    #[response(status = 404)]
    UnknownTaxi(Json<Failure>),
    #[response(status = 404)]
    UnknownStand(Json<Failure>),
    #[response(status = 404)]
    UnknownJourney(Json<Failure>),
    #[response(status = 400)]
    NoCredentials(Json<Failure>),
    #[response(status = 400)]
    UnableToCancelJourney(Json<Failure>),
    #[response(status = 400)]
    HasInProgressJourney(Json<Failure>),
    #[response(status = 404)]
    NoInProgressJourney(Json<Failure>),
}

impl Error {
    pub fn server_error() -> Self {
        Error::ServerError(Json(Failure {
            error: "server_error",
            error_description: String::from("Oops! Something went wrong..."),
        }))
    }

    pub fn taxi_exists(error_description: String) -> Self {
        Error::TaxiExists(Json(Failure {
            error: "invalid_request",
            error_description,
        }))
    }

    pub fn unknown_registration_id(id: &Uuid) -> Self {
        Error::UnknownRegistrationId(Json(Failure {
            error: "not_found",
            error_description: format!("The processing registration `{id}` does not exists`"),
        }))
    }
    pub fn invalid_number(num: &str) -> Self {
        Error::InvalidNumber(Json(Failure {
            error: "invalid_request",
            error_description: format!("The taxi number `{num}` does not exits"),
        }))
    }

    pub fn invalid_password() -> Self {
        Error::InvalidPassword(Json(Failure {
            error: "invalid_request",
            error_description: format!("The password is incorrect"),
        }))
    }

    pub fn no_credentials() -> Self {
        Error::NoCredentials(Json(Failure {
            error: "no_credentials",
            error_description: format!("You must be authenticated"),
        }))
    }

    pub fn unknown_taxi(num: &str) -> Self {
        Error::UnknownTaxi(Json(Failure {
            error: "not_found",
            error_description: format!("The taxi `{num}` does not exists"),
        }))
    }

    pub fn unknown_stand(id: &Uuid) -> Self {
        Error::UnknownStand(Json(Failure {
            error: "not_found",
            error_description: format!("The stand `{id}` does not exists"),
        }))
    }

    pub fn unknown_journey(id: &Uuid) -> Self {
        Error::UnknownJourney(Json(Failure {
            error: "not_found",
            error_description: format!("The journey `{id}` does not exists"),
        }))
    }

    pub fn unable_to_cancel_journey(id: &Uuid) -> Self {
        Error::UnableToCancelJourney(Json(Failure {
            error: "invalid_request",
            error_description: format!("The journey `{id}` has booking(s)"),
        }))
    }

    pub fn has_in_progress_journey() -> Self {
        Error::HasInProgressJourney(Json(Failure {
            error: "invalid_request",
            error_description: format!("You already have a journey in progress"),
        }))
    }

    pub fn no_in_progress_journey() -> Self {
        Error::NoInProgressJourney(Json(Failure {
            error: "invalid_request",
            error_description: format!("You have no journey in progress"),
        }))
    }
}
