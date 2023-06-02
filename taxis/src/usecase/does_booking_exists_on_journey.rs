use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait DoesBookingExistsOnJourney {
    async fn does_booking_exists_on_journey(&mut self, journey_id: &Uuid) -> sqlx::Result<bool>;
}
