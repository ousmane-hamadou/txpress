use async_trait::async_trait;

#[async_trait]
pub trait HasAJourneyInProgress {
    async fn has_a_journey_in_progress(&mut self, num: &str) -> sqlx::Result<bool>;
}
