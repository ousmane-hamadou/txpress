use async_trait::async_trait;

use crate::entity::{Station, StationId};

#[async_trait]
pub trait GetStand {
    async fn get_stand(&mut self, id: &StationId) -> Option<Station>;
}
