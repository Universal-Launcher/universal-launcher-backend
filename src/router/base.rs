use actix_web::{web, Scope};

use crate::handlers::base;

pub fn register() -> Scope {
    web::scope("/").service(web::resource("").route(web::get().to(base::index)))
}
