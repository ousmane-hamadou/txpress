use rocket::http::{Header, Status};
use rocket::response::Responder;
use rocket::{options, routes, Request, Response, Route};
use std::path::PathBuf;

pub const ACCESS_CONTROL_ALLOW_HEADERS: &'static str = "Access-Control-Allow-Headers";
pub const ACCESS_CONTROL_ALLOW_METHODS: &'static str = "Access-Control-Allow-Methods";
pub const ACCESS_CONTROL_ALLOW_ORIGIN: &'static str = "Access-Control-Allow-Origin";
pub const ACCESS_CONTROL_ALLOW_CREDENTIALS: &'static str = "Access-Control-Allow-Credentials";

#[options("/searches/<_p..>")]
fn searches(_p: PathBuf) -> CORS<Status> {
    CORS::from([
        (CORSKey::AllowMethods, "POST"),
        (CORSKey::AllowHeaders, "Content-Type"),
        (CORSKey::AllowOrigins, "http://localhost:3000"),
        (CORSKey::AllowCredentials, "true"),
    ])
}

enum CORSKey {
    AllowHeaders,
    AllowOrigins,
    AllowMethods,
    AllowCredentials,
}

pub fn cors() -> Vec<Route> {
    routes![searches]
}

pub struct CORS<T> {
    inner: T,
    allow_origin: Option<Header<'static>>,
    allow_headers: Option<Header<'static>>,
    allow_methods: Option<Header<'static>>,
    allow_credentials: Option<Header<'static>>,
}

impl<'r> Responder<'r, 'static> for CORS<Status> {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut resp = Response::build();

        if let Some(h) = self.allow_methods {
            resp.header(h);
        }
        if let Some(h) = self.allow_origin {
            resp.header(h);
        }
        if let Some(h) = self.allow_headers {
            resp.header(h);
        }
        if let Some(h) = self.allow_credentials {
            resp.header(h);
        }

        resp.status(self.inner);
        resp.ok()
    }
}

impl<const N: usize> From<[(CORSKey, &'static str); N]> for CORS<Status> {
    fn from(value: [(CORSKey, &'static str); N]) -> Self {
        let mut cors = CORS::new();

        value.into_iter().for_each(|(k, v)| match k {
            CORSKey::AllowHeaders => {
                cors.allow_headers = Some(Header::new(ACCESS_CONTROL_ALLOW_HEADERS, v))
            }
            CORSKey::AllowMethods => {
                cors.allow_methods = Some(Header::new(ACCESS_CONTROL_ALLOW_METHODS, v))
            }
            CORSKey::AllowOrigins => {
                cors.allow_origin = Some(Header::new(ACCESS_CONTROL_ALLOW_ORIGIN, v))
            }
            CORSKey::AllowCredentials => {
                cors.allow_credentials = Some(Header::new(ACCESS_CONTROL_ALLOW_CREDENTIALS, v))
            }
        });
        cors
    }
}

impl CORS<Status> {
    fn new() -> Self {
        CORS {
            inner: Status::Ok,
            allow_methods: None,
            allow_headers: None,
            allow_origin: None,
            allow_credentials: None,
        }
    }
}
