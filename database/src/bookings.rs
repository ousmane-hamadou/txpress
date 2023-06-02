use crate::TXpressDB;
use async_trait::async_trait;
use database_derive::DataProvider;
use domain::entity::Booking;
use domain::usecase::booking::PerformBooking;
use rocket_db_pools::Connection;
use uuid::Uuid;

#[derive(DataProvider)]
struct Bookings(Connection<TXpressDB>);

impl Bookings {
    fn new(conn: Connection<TXpressDB>) -> Self {
        Bookings(conn)
    }
}

#[async_trait]
impl PerformBooking for Bookings {
    async fn perform_booking(&mut self, _book: &Booking) -> bool {
        todo!()
    }
}
