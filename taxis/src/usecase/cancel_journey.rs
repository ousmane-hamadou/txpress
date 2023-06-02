use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CancelJourney {
    async fn cancel_journey(&mut self, journey_id: &Uuid) -> sqlx::Result<()>;
}
