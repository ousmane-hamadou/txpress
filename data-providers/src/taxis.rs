use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::error;
use rocket_db_pools::Connection;
use sqlx::{Acquire, Postgres, Transaction};
use uuid::Uuid;

use database_derive::DataProvider;
use domain::entity::{DepartureSchedule, JourneyCriteria, JourneyId, Taxi, TaxiOwner};
use domain::usecase::taxi::{
    AddTaxi, GetJourneyDepartureSchedule, GetTaxi, GetTaxiOwner, PerformJourney,
};

use crate::TXpressDB;

const TARGET: &'static str = "taxi-data-provider";

#[derive(DataProvider)]
pub struct Taxis(Connection<TXpressDB>);

impl Taxis {
    pub fn new(conn: Connection<TXpressDB>) -> Self {
        Taxis(conn)
    }
}

#[async_trait]
impl GetTaxi for Taxis {
    async fn get_taxi(&mut self, num: &str) -> Option<Taxi> {
        sqlx::query_as!(
            Taxi,
            "SELECT number, brand, number_of_seats FROM taxis WHERE number = lower($1)",
            num,
        )
        .fetch_optional(&mut *self.0)
        .await
        .map(|taxi| taxi.map(|t| t.into()))
        .unwrap_or_else(|err| {
            error!(target: "taxi-data-provider", "unable to fetch taxi: {:?}", err);
            None
        })
    }
}

#[async_trait]
impl GetTaxiOwner for Taxis {
    async fn get_taxi_owner(&mut self, taxi_num: &str) -> Option<TaxiOwner> {
        sqlx::query_as!(
            TaxiOwner,
            "SELECT full_name, password FROM taxi_owners WHERE id = lower($1)",
            taxi_num,
        )
        .fetch_optional(&mut *self.0)
        .await
        .unwrap_or_else(|err| {
            error!(target: TARGET, "unable to fetch taxi owner\n{err:?}");
            None
        })
    }
}

#[async_trait]
impl PerformJourney for Taxis {
    async fn perform_journey(
        &mut self,
        number: &str,
        criteria: &JourneyCriteria,
    ) -> Option<JourneyId> {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO trips(id, owner, origin, destination, departure_schedule)
                VALUES ($1, lower($2), $3, $4, $5)",
            id,
            number,
            &criteria.origin,
            &criteria.destination,
            &criteria.departure_schedule,
        )
        .execute(&mut *self.0)
        .await
        .map(|_| Some(id))
        .unwrap_or_else(|err| {
            error!(target: TARGET, "unable to create journey\n{err:?}");
            None
        })
    }
}

struct Departure {
    departure_schedule: DateTime<Utc>,
}

#[async_trait]
impl GetJourneyDepartureSchedule for Taxis {
    async fn get_journey_departure_schedule(&mut self, id: JourneyId) -> Option<DepartureSchedule> {
        sqlx::query_as!(
            Departure,
            "SELECT departure_schedule from trips WHERE id = $1",
            id
        )
        .map(|d| d.departure_schedule)
        .fetch_optional(&mut *self.0)
        .await
        .unwrap_or_else(|err| {
            error!(
                target: TARGET,
                "unable to fetch departure_schedule\n{err:?}"
            );
            None
        })
    }
}

#[async_trait]
impl AddTaxi for Taxis {
    async fn add_taxi(&mut self, taxi: &Taxi, owner: &TaxiOwner) -> bool {
        let mut tx = self.0.begin().await.unwrap();

        match insert_taxi(&mut tx, taxi).await {
            Err(err) => {
                error!(target: TARGET, "unable to add taxi\n{err:?}");
                false
            }
            _ => match insert_owner(&mut tx, &taxi.number, owner).await {
                Err(err) => {
                    error!(target: TARGET, "unable to add taxi owner\n{err:?}");
                    false
                }
                _ => {
                    tx.commit().await.unwrap();
                    true
                }
            },
        }
    }
}

async fn insert_taxi(tx: &mut Transaction<'_, Postgres>, taxi: &Taxi) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO taxis (number, brand, number_of_seats) VALUES (lower($1), $2, $3)",
        taxi.number,
        &taxi.brand,
        taxi.number_of_seats as i32,
    )
    .execute(tx)
    .await?;

    Ok(())
}

async fn insert_owner(
    tx: &mut Transaction<'_, Postgres>,
    taxi_num: &str,
    owner: &TaxiOwner,
) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO taxi_owners (id, full_name, password) VALUES (lower($1), $2, $3)",
        taxi_num,
        &owner.full_name,
        &owner.password,
    )
    .execute(tx)
    .await?;

    Ok(())
}
