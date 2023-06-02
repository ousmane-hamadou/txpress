use async_trait::async_trait;

use crate::entity::Owner;

#[async_trait]
pub trait GetOwner {
    async fn get_owner(&mut self, num: &str) -> sqlx::Result<Option<Owner>>;
}