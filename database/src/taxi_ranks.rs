use async_trait::async_trait;
use log::error;
use rocket_db_pools::Connection;
use uuid::Uuid;

use database_derive::DataProvider;
use domain::entity::{Station, StationId};
use domain::usecase::stand::{AddStand, GetAllStand, GetStand};

use crate::TXpressDB;

const TARGET: &'static str = "stand-data-provider";

#[derive(DataProvider)]
pub struct TaxiRanks(Connection<TXpressDB>);

impl TaxiRanks {
    fn new(conn: Connection<TXpressDB>) -> Self {
        TaxiRanks(conn)
    }
}

#[async_trait]
impl GetAllStand for TaxiRanks {
    async fn get_all_stand(&mut self) -> Vec<Station> {
        sqlx::query_as!(Station, "SELECT id, name FROM taxi_ranks")
            .fetch_all(&mut *self.0)
            .await
            .unwrap_or_else(|err| {
                error!(target: TARGET, "unable to fetch stand\n{err:?}");
                vec![]
            })
    }
}

#[async_trait]
impl GetStand for TaxiRanks {
    async fn get_stand(&mut self, id: &StationId) -> Option<Station> {
        sqlx::query_as!(Station, "SELECT id, name FROM taxi_ranks WHERE id = $1", id)
            .fetch_optional(&mut *self.0)
            .await
            .unwrap_or_else(|err| {
                error!(target: TARGET, "unable to fetch stand\n{err:?}");
                None
            })
    }
}

#[async_trait]
impl AddStand for TaxiRanks {
    async fn add_stand(&mut self, station_names: Vec<String>) -> Vec<Station> {
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
                .map(|(name, id)| Station { id, name })
                .collect::<Vec<Station>>()
        })
        .unwrap_or_else(|err| {
            error!(target: TARGET, "unable to insert stand\n{err:?}");
            vec![]
        })
    }
}
