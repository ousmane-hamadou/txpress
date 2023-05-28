use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SearchedTaxi {
    pub number: String,
    pub journey_id: Uuid,
    pub brand: String,
    pub number_of_seats: i32,
    pub available_seats: i32,
    pub departure_schedule: DateTime<Utc>,
}
