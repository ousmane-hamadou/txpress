use async_trait::async_trait;
use crate::entity::Journey;

#[async_trait]
pub trait GetAllJourney {
    async fn get_all_journey(&mut self, taxi_num: &str) -> sqlx::Result<Vec<Journey>>;
}
