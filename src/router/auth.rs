use actix_web::{web, Scope};

use crate::handlers::auth;

pub fn register() -> Scope {
    web::scope("/auth").service(web::resource("").route(web::get().to(auth::index)))
}
