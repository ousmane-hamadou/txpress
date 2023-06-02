use async_trait::async_trait;

use crate::entity::Stand;

#[async_trait]
pub trait AddTaxiRanks {
    async fn add_taxi_ranks(&mut self, stand_names: Vec<String>) -> sqlx::Result<Vec<Stand>>;
}
