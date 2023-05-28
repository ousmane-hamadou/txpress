use async_trait::async_trait;

use crate::entity::TaxiOwner;

#[async_trait]
pub trait GetTaxiOwner {
    async fn get_taxi_owner(&mut self, taxi_num: &str) -> Option<TaxiOwner>;
}
