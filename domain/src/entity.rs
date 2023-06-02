use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use booking::Booking;
pub use journey::Journey;
pub use journey_criteria::JourneyCriteria;
pub use price::Price;
pub use search::Search;
pub use search_criteria::SearchCriteria;
pub use searched_taxi::SearchedTaxi;
pub use station::Station;
pub use taxi::Taxi;
pub use taxi_owner::TaxiOwner;
pub use taxi_with_owner::TaxiWithOwner;

mod booking;
mod journey;
mod journey_criteria;
mod price;
mod search;
mod search_criteria;
mod searched_taxi;
mod station;
mod taxi;
mod taxi_owner;
mod taxi_with_owner;

pub type JourneyId = Uuid;
pub type StationId = Uuid;
pub type TaxiNumber = String;
pub type DepartureSchedule = DateTime<Utc>;
pub type BookingId = Uuid;
pub type StationName = String;
