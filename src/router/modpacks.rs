use actix_web::{web, Scope};

use crate::handlers::modpacks;

pub fn register() -> Scope {
    web::scope("modpacks").service(web::resource("").route(web::get().to(modpacks::index)))
}
