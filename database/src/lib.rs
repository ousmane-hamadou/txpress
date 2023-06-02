use rocket_db_pools::{Database, Initializer};

#[derive(Database)]
#[database("txpress")]
pub struct TXpressDB(sqlx::PgPool);

pub fn init() -> Initializer<TXpressDB> {
    TXpressDB::init()
}
