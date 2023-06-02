use async_trait::async_trait;

use crate::entity::Stand;

#[async_trait]
pub trait GetAllStand {
    async fn get_all_stand(&mut self) -> sqlx::Result<Vec<Stand>>;
}
