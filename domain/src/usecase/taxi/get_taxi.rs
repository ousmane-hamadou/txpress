use async_trait::async_trait;

use crate::entity::Taxi;

#[async_trait]
pub trait GetTaxi {
    async fn get_taxi(&mut self, num: &str) -> Option<Taxi>;
}
