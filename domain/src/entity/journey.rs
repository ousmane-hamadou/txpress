use crate::entity::{BookingId, JourneyCriteria, JourneyId, TaxiNumber};

pub struct Journey {
    pub id: JourneyId,
    pub owner: TaxiNumber,
    pub criteria: JourneyCriteria,
    pub bookings: Vec<BookingId>,
}
