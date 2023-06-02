use crate::entity::Journey;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait GetJourney {
    async fn get_journey(&mut self, id: &Uuid) -> sqlx::Result<Option<Journey>>;
}
