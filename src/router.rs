use actix_web::error::QueryPayloadError;
use actix_web::{error, web, HttpResponse};
use serde::Serialize;

mod auth;
mod base;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::register()).service(base::register());

    configure_errors(cfg);
}

#[derive(Debug, Serialize)]
struct ErrorHandling {
    error: String,
}

fn configure_errors(cfg: &mut web::ServiceConfig) {
    // Query string default configuration
    cfg.app_data(
        web::QueryConfig::default().error_handler(|err: QueryPayloadError, _| {
            let resp = HttpResponse::BadRequest().json(ErrorHandling {
                error: String::from(err.to_string()),
            });
            error::InternalError::from_response(err, resp).into()
        }),
    );
}
