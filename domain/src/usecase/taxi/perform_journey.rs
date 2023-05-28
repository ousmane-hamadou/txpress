use async_trait::async_trait;

use crate::entity::JourneyCriteria;
use crate::entity::JourneyId;

#[async_trait]
pub trait PerformJourney {
    async fn perform_journey(
        &mut self,
        taxi_id: &str,
        criteria: &JourneyCriteria,
    ) -> Option<JourneyId>;
}
