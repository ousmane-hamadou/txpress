use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct JourneyCriteria {
    pub origin: Uuid,
    pub destination: Uuid,
    pub departure_schedule: DateTime<Utc>,
}
