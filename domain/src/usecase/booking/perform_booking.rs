use crate::entity::Booking;
use async_trait::async_trait;

#[async_trait]
pub trait PerformBooking {
    async fn perform_booking(&mut self, book: &Booking) -> bool;
}
