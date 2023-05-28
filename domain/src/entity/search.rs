use crate::entity::Taxi;
use uuid::Uuid;

pub struct Search {
    pub id: Uuid,
    pub available_taxis: Vec<Taxi>,
    pub selection: Option<String>,
}
