use async_trait::async_trait;
use uuid::Uuid;

use crate::entity::Stand;

#[async_trait]
pub trait GetStand {
    async fn get_stand(&mut self, id: &Uuid) -> sqlx::Result<Option<Stand>>;
}
