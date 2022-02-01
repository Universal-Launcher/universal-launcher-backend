use actix_web::{web, Scope};

use crate::handlers::modpacks;

pub fn register() -> Scope {
    web::scope("/modpacks")
        .service(
            web::resource("/")
                .route(web::get().to(modpacks::index))
                .route(web::post().to(modpacks::create)),
        )
        .service(
            web::resource("/{m_id}")
                .route(web::get().to(modpacks::show))
                .route(web::put().to(modpacks::update))
                .route(web::delete().to(modpacks::delete)),
        )
}
