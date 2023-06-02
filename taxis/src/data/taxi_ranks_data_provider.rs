use async_trait::async_trait;
use database::TXpressDB;
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::entity::Stand;
use crate::usecase::{AddTaxiRanks, GetAllStand, GetStand};
use database_derive::DataProvider;

#[derive(DataProvider)]
pub struct StandDataProvider(Connection<TXpressDB>);

impl StandDataProvider {
    fn new(conn: Connection<TXpressDB>) -> Self {
        StandDataProvider(conn)
    }
}

#[async_trait]
impl GetAllStand for StandDataProvider {
    async fn get_all_stand(&mut self) -> sqlx::Result<Vec<Stand>> {
        sqlx::query_as!(Stand, "SELECT id, name FROM taxi_ranks")
            .fetch_all(&mut *self.0)
            .await
    }
}

#[async_trait]
impl GetStand for StandDataProvider {
    async fn get_stand(&mut self, id: &Uuid) -> sqlx::Result<Option<Stand>> {
        sqlx::query_as!(Stand, "SELECT id, name FROM taxi_ranks WHERE id = $1", id)
            .fetch_optional(&mut *self.0)
            .await
    }
}

#[async_trait]
impl AddTaxiRanks for StandDataProvider {
    async fn add_taxi_ranks(&mut self, station_names: Vec<String>) -> sqlx::Result<Vec<Stand>> {
        let mut ids = Vec::with_capacity(station_names.len());
        for _ in 0..station_names.len() {
            ids.push(Uuid::new_v4())
        }

        sqlx::query!(
            "INSERT INTO taxi_ranks (id, name) SELECT * FROM UNNEST($1::uuid[], $2::text[])",
            &ids,
            &station_names
        )
        .execute(&mut *self.0)
        .await
        .map(|_| {
            station_names
                .into_iter()
                .zip(ids)
                .map(|(name, id)| Stand { id, name })
                .collect::<Vec<Stand>>()
        })
    }
}
