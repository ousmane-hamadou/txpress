use async_trait::async_trait;

use crate::entity::{DepartureSchedule, JourneyId};

#[async_trait]
pub trait GetJourneyDepartureSchedule {
    async fn get_journey_departure_schedule(&mut self, id: JourneyId) -> Option<DepartureSchedule>;
}
