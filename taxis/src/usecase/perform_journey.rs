use async_trait::async_trait;
use uuid::Uuid;

use crate::entity::JourneyCriteria;

#[async_trait]
pub trait PerformJourney {
    async fn perform_journey(
        &mut self,
        taxi_id: &str,
        criteria: &JourneyCriteria,
    ) -> sqlx::Result<Uuid>;
}
