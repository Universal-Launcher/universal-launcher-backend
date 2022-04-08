use actix_web::{web, Resource};

use crate::handlers::base;

pub fn register() -> Resource {
    web::resource("/").route(web::get().to(base::index))
}
