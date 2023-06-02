use async_trait::async_trait;
use rocket_db_pools::Connection;

use database::TXpressDB;
use database_derive::DataProvider;

use crate::entity::Owner;
use crate::usecase::GetOwner;

#[derive(DataProvider)]
pub struct LoginDataProvider(Connection<TXpressDB>);

impl LoginDataProvider {
    fn new(conn: Connection<TXpressDB>) -> Self {
        LoginDataProvider(conn)
    }
}

#[async_trait]
impl GetOwner for LoginDataProvider {
    async fn get_owner(&mut self, num: &str) -> sqlx::Result<Option<Owner>> {
        sqlx::query_as!(
            Owner,
            "SELECT full_name, password FROM taxi_owners WHERE id = lower($1)",
            num
        )
        .fetch_optional(&mut *self.0)
        .await
    }
}
