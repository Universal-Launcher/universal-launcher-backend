use actix_web::{web, Scope};

use crate::handlers::user;

pub fn register() -> Scope {
    web::scope("/user")
        .service(web::resource("/auth-redirect").route(web::get().to(user::redirect)))
        .service(web::resource("/fallback").route(web::get().to(user::fallback)))
}
