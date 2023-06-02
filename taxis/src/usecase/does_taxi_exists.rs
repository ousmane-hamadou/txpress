use async_trait::async_trait;

#[async_trait]
pub trait DoesTaxiExists {
    async fn does_taxi_exists(&mut self, num: &str) -> sqlx::Result<bool>;
}
