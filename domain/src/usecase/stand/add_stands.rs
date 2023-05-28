use async_trait::async_trait;

use crate::entity::Station;

#[async_trait]
pub trait AddStand {
    async fn add_stand(&mut self, stand_names: Vec<String>) -> Vec<Station>;
}
