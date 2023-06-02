use async_trait::async_trait;
use rocket_db_pools::Connection;
use uuid::Uuid;

use database::TXpressDB;
use database_derive::DataProvider;

use crate::entity::{Journey, JourneyCriteria};
use crate::usecase::{
    CancelJourney, CloseJourney, DoesBookingExistsOnJourney, GetAllJourney, GetInProgressJourney,
    GetJourney, HasAJourneyInProgress, PerformJourney,
};

#[derive(DataProvider)]
pub struct JourneyDataProvider(Connection<TXpressDB>);

impl JourneyDataProvider {
    fn new(conn: Connection<TXpressDB>) -> Self {
        JourneyDataProvider(conn)
    }
}

#[async_trait]
impl GetJourney for JourneyDataProvider {
    async fn get_journey(&mut self, id: &Uuid) -> sqlx::Result<Option<Journey>> {
        sqlx::query_as!(
            Journey,
            "SELECT id, origin, destination, reserved_seats, departure_schedule, closed FROM trips
            WHERE id = $1",
            id
        )
        .fetch_optional(&mut *self.0)
        .await
    }
}

#[async_trait]
impl GetAllJourney for JourneyDataProvider {
    async fn get_all_journey(&mut self, taxi_num: &str) -> sqlx::Result<Vec<Journey>> {
        sqlx::query_as!(Journey,
           "SELECT tp.id, tp.origin, tp.destination, tp.reserved_seats, tp.departure_schedule, tp.closed FROM trips tp
                WHERE tp.owner = lower($1)
            ORDER BY tp.departure_schedule DESC", taxi_num)
           .fetch_all(&mut *self.0).await
    }
}

#[async_trait]
impl GetInProgressJourney for JourneyDataProvider {
    async fn get_in_progress_journey(&mut self, owner: &str) -> sqlx::Result<Option<Journey>> {
        sqlx::query_as!(
            Journey,
            "SELECT id, origin, destination, reserved_seats, departure_schedule, closed FROM trips
            WHERE owner = $1",
            owner
        )
        .fetch_optional(&mut *self.0)
        .await
    }
}

#[async_trait]
impl PerformJourney for JourneyDataProvider {
    async fn perform_journey(
        &mut self,
        taxi_num: &str,
        criteria: &JourneyCriteria,
    ) -> sqlx::Result<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO trips(id, owner, origin, destination, departure_schedule)
                VALUES ($1, lower($2), $3, $4, $5)",
            &id,
            taxi_num,
            &criteria.origin,
            &criteria.destination,
            &criteria.departure_schedule,
        )
        .execute(&mut *self.0)
        .await?;

        Ok(id)
    }
}

#[async_trait]
impl CloseJourney for JourneyDataProvider {
    async fn close_journey(&mut self, journey_id: &Uuid) -> sqlx::Result<()> {
        sqlx::query!("UPDATE trips SET closed = TRUE WHERE id = $1", journey_id)
            .execute(&mut *self.0)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl CancelJourney for JourneyDataProvider {
    async fn cancel_journey(&mut self, journey_id: &Uuid) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM trips WHERE id = $1", journey_id)
            .execute(&mut *self.0)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DoesBookingExistsOnJourney for JourneyDataProvider {
    async fn does_booking_exists_on_journey(&mut self, journey_id: &Uuid) -> sqlx::Result<bool> {
        Ok(sqlx::query!(
            "SELECT 1 AS n FROM bookings WHERE journey_id = $1",
            journey_id,
        )
        .fetch_optional(&mut *self.0)
        .await?
        .is_some())
    }
}

#[async_trait]
impl HasAJourneyInProgress for JourneyDataProvider {
    async fn has_a_journey_in_progress(&mut self, num: &str) -> sqlx::Result<bool> {
        Ok(sqlx::query!(
            "SELECT 1 as n FROM trips WHERE owner = $1 AND closed = FALSE",
            num
        )
        .fetch_optional(&mut *self.0)
        .await?
        .is_some())
    }
}
