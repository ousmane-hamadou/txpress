use rocket_db_pools::{Database, Initializer};

pub use searches::Searches;
pub use taxi_ranks::TaxiRanks;
pub use taxis::Taxis;

mod searches;
pub mod taxi_ranks;
pub mod taxis;

#[derive(Database)]
#[database("txpress")]
pub struct TXpressDB(sqlx::PgPool);

pub fn init() -> Initializer<TXpressDB> {
    TXpressDB::init()
}
