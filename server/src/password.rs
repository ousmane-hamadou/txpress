use bcrypt::BcryptResult;
use rocket::tokio::task;

pub async fn hash(password: String) -> BcryptResult<String> {
    task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
        .await
        .unwrap()
}

pub async fn verify(password: &str, hash: &str) -> BcryptResult<bool> {
    let hash = hash.to_owned();
    let password = password.to_owned();

    task::spawn_blocking(move || bcrypt::verify(password.to_owned(), &hash))
        .await
        .unwrap()
}
