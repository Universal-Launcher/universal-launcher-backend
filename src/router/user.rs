use actix_web::{web, Scope};

use crate::handlers::user;

pub fn register() -> Scope {
    web::scope("/user")
        .service(
            web::resource("/auth")
                .route(web::post().to(user::authenticate_user))
                .route(web::get().to(user::get_user))
                .route(web::delete().to(user::logout_user)),
        )
        .service(web::resource("/auth/create").route(web::post().to(user::register_user)))
}
