use crate::errors::Error;
use log::error;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TaxiOwner {
    pub full_name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TaxiOwner {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res = request.param::<&str>(1).unwrap().map(|taxi_num| {
            request
                .cookies()
                .get_private(taxi_num)
                .map(|c| serde_json::from_str::<TaxiOwner>(c.value()))
                .and_then(|owner| {
                    if let Err(err) = owner {
                        error!(target: "taxi-owner-guard", "{err:?}");
                        return None;
                    }

                    Some(owner.unwrap())
                })
        });

        if let Err(_) = res {
            return Outcome::Failure((
                Status::BadRequest,
                Error::UnknownTaxi(request.param(1).unwrap().unwrap()),
            ));
        }

        match res.unwrap() {
            None => Outcome::Failure((Status::Unauthorized, Error::NoCredentials)),
            Some(t) => Outcome::Success(t),
        }
    }
}
