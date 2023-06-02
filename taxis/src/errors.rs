#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an internal server error occurred")]
    Sqlx(#[from] sqlx::Error),
}
