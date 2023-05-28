use async_trait::async_trait;
use uuid::Uuid;

use crate::entity::SearchedTaxi;

#[async_trait]
pub trait GetSelectedTaxi {
    async fn get_selected_taxi(&mut self, journey_id: &Uuid) -> SearchedTaxi;
}
