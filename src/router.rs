use actix_web::web;

mod auth;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::register());
}
