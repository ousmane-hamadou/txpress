use async_trait::async_trait;

use crate::entity::{SearchCriteria, SearchedTaxi};

#[async_trait]
pub trait FetchTaxiByJourneyCriteria {
    async fn fetch_taxis(&mut self, criteria: &SearchCriteria) -> Vec<SearchedTaxi>;
}
