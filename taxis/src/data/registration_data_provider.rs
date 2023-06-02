use async_trait::async_trait;
use rocket_db_pools::Connection;
use sqlx::Acquire;

use database::TXpressDB;
use database_derive::DataProvider;

use crate::entity::{Owner, Taxi};
use crate::usecase::{AddTaxi, DoesTaxiExists};

#[derive(DataProvider)]
pub struct RegistrationDataProvider(Connection<TXpressDB>);

impl RegistrationDataProvider {
    fn new(conn: Connection<TXpressDB>) -> Self {
        RegistrationDataProvider(conn)
    }
}

#[async_trait]
impl AddTaxi for RegistrationDataProvider {
    async fn add_taxi(&mut self, taxi: &Taxi, owner: &Owner) -> sqlx::Result<()> {
        let mut tx = self.0.begin().await?;

        sqlx::query!(
            "INSERT INTO taxis (id, number, brand, number_of_seats) VALUES ($1,lower($2), $3, $4)",
            taxi.id,
            &taxi.number,
            taxi.brand,
            taxi.number_of_seats,
        )
        .execute(&mut tx)
        .await?;

        sqlx::query!(
            "INSERT INTO taxi_owners (id, full_name, password) VALUES (lower($1), $2, $3)",
            taxi.number,
            owner.full_name,
            owner.password,
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl DoesTaxiExists for RegistrationDataProvider {
    async fn does_taxi_exists(&mut self, num: &str) -> sqlx::Result<bool> {
        Ok(
            sqlx::query!("SELECT 1 as n FROM taxis WHERE number = lower($1)", num)
                .fetch_optional(&mut *self.0)
                .await?
                .is_some(),
        )
    }
}
