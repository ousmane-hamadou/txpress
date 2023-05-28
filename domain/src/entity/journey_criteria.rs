use crate::entity::{DepartureSchedule, StationId};

pub struct JourneyCriteria {
    pub origin: StationId,
    pub destination: StationId,
    pub departure_schedule: DepartureSchedule,
}
