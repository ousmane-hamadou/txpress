use server::server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server().launch().await?;
    Ok(())
}
