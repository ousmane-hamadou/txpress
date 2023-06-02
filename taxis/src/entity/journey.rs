use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Journey {
    pub id: Uuid,
    pub origin: Uuid,
    pub destination: Uuid,
    pub reserved_seats: i32,
    pub departure_schedule: DateTime<Utc>,
    pub closed: bool,
}
