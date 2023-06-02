use crate::entity::{Owner, Taxi};
use async_trait::async_trait;

#[async_trait]
pub trait AddTaxi {
    async fn add_taxi(&mut self, taxi: &Taxi, owner: &Owner) -> sqlx::Result<()>;
}
