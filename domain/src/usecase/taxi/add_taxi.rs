use async_trait::async_trait;

use crate::entity::Taxi;
use crate::entity::TaxiOwner;

#[async_trait]
pub trait AddTaxi {
    async fn add_taxi(&mut self, taxi: &Taxi, owner: &TaxiOwner) -> bool;
}
