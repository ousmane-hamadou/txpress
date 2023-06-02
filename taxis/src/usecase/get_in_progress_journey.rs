use async_trait::async_trait;

use crate::entity::Journey;

#[async_trait]
pub trait GetInProgressJourney {
    async fn get_in_progress_journey(&mut self, owner: &str) -> sqlx::Result<Option<Journey>>;
}
