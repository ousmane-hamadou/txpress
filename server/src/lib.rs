mod cors;

use rocket::{routes, Build, Rocket};

pub fn server() -> Rocket<Build> {
    rocket::build()
        .attach(database::init())
        .attach(cors::CORS)
        .mount("/", routes![cors::for_cors])
        .mount("/", taxis::routes())
}
