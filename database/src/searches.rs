use async_trait::async_trait;
use log::error;
use rocket_db_pools::Connection;
use uuid::Uuid;

use database_derive::DataProvider;
use domain::entity::{SearchCriteria, SearchedTaxi};
use domain::usecase::search::{FetchTaxiByJourneyCriteria, GetSelectedTaxi};

use crate::TXpressDB;

const TARGET: &'static str = "searches-data-provider";

#[derive(DataProvider)]
pub struct Searches(Connection<TXpressDB>);

impl Searches {
    fn new(conn: Connection<TXpressDB>) -> Self {
        Searches(conn)
    }
}

#[async_trait]
impl FetchTaxiByJourneyCriteria for Searches {
    async fn fetch_taxis(&mut self, criteria: &SearchCriteria) -> Vec<SearchedTaxi> {
        sqlx::query_as!(
            SearchedTaxi,
            "SELECT tp.id as journey_id, t.number, t.brand, t.number_of_seats, tp.departure_schedule, tp.available_seats
                FROM trips tp
                    INNER JOIN taxis t ON t.number = tp.owner
                        WHERE tp.origin = $1 AND tp.destination = $2 AND tp.finished = false",
            &criteria.departure,
            &criteria.arrival
        )
        .fetch_all(&mut *self.0)
        .await
        .unwrap_or_else(|err| {
            error!(target: TARGET, "unable to fetch taxis\n{err:?}");
            vec![]
        })
    }
}

#[async_trait]
impl GetSelectedTaxi for Searches {
    async fn get_selected_taxi(&mut self, journey_id: &Uuid) -> SearchedTaxi {
        sqlx::query_as!(
            SearchedTaxi,
            "SELECT tp.id as journey_id, t.number, t.brand, t.number_of_seats, tp.departure_schedule, tp.available_seats
                FROM trips tp
                    INNER JOIN taxis t ON t.number = tp.owner
                        WHERE tp.id = $1",
            journey_id,
        )
        .fetch_one(&mut *self.0)
        .await
        .unwrap()
    }
}
